use std::ops::Deref;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};
use std::time::{SystemTime, UNIX_EPOCH};
use packets::packets::{PacketZcItemDisappear, PacketZcItemFallEntry, PacketZcNotifyMove};
use crate::server::model::map::Map;

use crate::server::model::map_item::{MapItem, MapItemSnapshot, ToMapItem};
use crate::server::model::path::manhattan_distance;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::{MOB_FOV, Server};
use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::map_instance_loop::MAP_LOOP_TICK_RATE;
use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent};
use crate::server::model::events::map_event::MobDropItems;
use crate::server::model::item::DroppedItem;
use crate::server::model::position::Position;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::mob_service::MobService;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::{Mob, MobMovement};
use crate::server::state::status::Status;
use crate::util::tick::{delayed_tick, get_tick, get_tick_client};

static mut SERVICE_INSTANCE: Option<MapInstanceService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct MapInstanceService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    mob_service: MobService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

impl MapInstanceService {
    pub fn instance() -> &'static MapInstanceService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, mob_service: MobService, server_task_queue: Arc<TasksQueue<GameEvent>>) -> Self {
        MapInstanceService { client_notification_sender, configuration_service, mob_service, server_task_queue }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, mob_service: MobService, server_task_queue: Arc<TasksQueue<GameEvent>>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(MapInstanceService::new(client_notification_sender, configuration_service, mob_service, server_task_queue));
        });
    }

    pub fn spawn_mobs(&self, map: &Map, map_instance_state: &mut MapInstanceState) {
        for mob_spawn in map.mob_spawns().iter() {
            let mob_spawn_track = map_instance_state.mob_spawns_tracks_mut().iter().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap().clone();
            if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                continue;
            }
            if mob_spawn.has_delay() {
                // TODO check when respawn is planned
            }
            let mut cell: (u16, u16);
            let spawned = mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount;
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {
                    // if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(map_instance_state.cells_mut().deref(), map.x_size());
                }
                let mob_map_item_id = Server::generate_id(map_instance_state.map_items_mut());
                let mob = Mob::new(mob_map_item_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.info.name.clone(), mob_spawn.info.name_english.clone(),
                                   Status::from_mob_model(&mob_spawn.info));

                map_instance_state.insert_mob(mob);
                // END
                let mob_spawn_track = map_instance_state.mob_spawns_tracks_mut().iter_mut().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap();
                mob_spawn_track.increment_spawn();
            }
        }
    }

    pub fn update_mobs_fov(&self, map_instance_state: &mut MapInstanceState, characters: Vec<MapItemSnapshot>) {
        for (_, mob) in map_instance_state.mobs_mut().iter_mut() {
            let mut viewed_chars: Vec<MapItem> = Vec::with_capacity(characters.len());
            for character in characters.iter() {
                if manhattan_distance(character.x(), character.y(), mob.x(), mob.y()) <= MOB_FOV {
                    viewed_chars.push(character.map_item());
                }
            }
            mob.update_map_view(viewed_chars);
        }
    }

    pub fn mobs_action(&self, map_instance_state: &mut MapInstanceState) {
        let start_time = get_tick_client();
        let start_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut mob_movements: Vec<MobMovement> = Vec::with_capacity(map_instance_state.mobs().len() / 2);
        let cells = map_instance_state.cells().clone();
        let x_size = map_instance_state.x_size();
        let y_size = map_instance_state.y_size();
        for mob in map_instance_state.mobs_mut().values_mut() {
            if let Some(mob_movement) = self.mob_service.action_move(mob, cells.as_ref(), x_size, y_size, start_at) {
                mob_movements.push(mob_movement);
            }
        }
        for mob_movement in mob_movements {
            let mut packet_zc_notify_move = PacketZcNotifyMove::default();
            packet_zc_notify_move.set_gid(mob_movement.id);
            packet_zc_notify_move.move_data = mob_movement.from.to_move_data(&mob_movement.to);
            packet_zc_notify_move.set_move_start_time(start_time);
            packet_zc_notify_move.fill_raw();
            debug!("Mob moving from {} to {}. Notify area around {},{}", mob_movement.from, mob_movement.to, mob_movement.from.x, mob_movement.from.y);
            self.client_notification_sender.send(Notification::Area(
                AreaNotification::new(map_instance_state.key().map_name().clone(), map_instance_state.key().map_instance(),
                                      AreaNotificationRangeType::Fov { x: mob_movement.from.x, y: mob_movement.from.y, exclude_id: None },
                                      packet_zc_notify_move.raw))).expect("Fail to send client notification");
        }
    }

    pub fn mob_die(&self, map_instance_state: &mut MapInstanceState, id: u32, delay: u128) {
        let mob = map_instance_state.remove_mob(id).unwrap();
        self.server_task_queue.add_to_index(GameEvent::CharacterKillMonster(CharacterKillMonster { char_id: mob.attacker_with_higher_damage(), mob_id: mob.mob_id, mob_x: mob.x, mob_y: mob.y, map_instance_key: map_instance_state.key().clone() }),
                                            delayed_tick(delay, GAME_TICK_RATE)
        );
    }

    pub fn mob_drop_items(&self, map_instance_state: &mut MapInstanceState, mob_drop_items: MobDropItems) -> Vec<DroppedItem> {
        let rng = fastrand::Rng::new();
        let mob = self.configuration_service.get_mob(mob_drop_items.mob_id as i32);
        let mut item_to_drop: Vec<DroppedItem> = vec![];
        for drop in mob.drops.iter() {
            let drop_rate = if drop.is_card {
                (drop.rate as f32 * self.configuration_service.config().game.drop_rate_card).round() as u16
            } else {
                (drop.rate as f32 * self.configuration_service.config().game.drop_rate).round() as u16
            };
            if drop_rate >= 10000 || rng.u16(1..=10000) > 10000 - drop_rate {
                let (random_x, random_y) = Map::find_random_free_cell_around(map_instance_state.cells(), map_instance_state.x_size(), mob_drop_items.mob_x, mob_drop_items.mob_y);
                let map_item_id = Server::generate_id(map_instance_state.map_items_mut());
                let dropped_item = DroppedItem {
                    map_item_id,
                    item_id: drop.item_id,
                    location: Position { x: random_x, y: random_y, dir: 0 },
                    sub_location: Position { x: rng.u16(0..=3) * 3 + 3, y: rng.u16(0..=3) * 3 + 3, dir: 0 },
                    owner_id: Some(mob_drop_items.owner_id),
                    dropped_at: get_tick(),
                    amount: 1
                };
                map_instance_state.insert_dropped_item(dropped_item);
                item_to_drop.push(dropped_item);
            }
        }
        item_to_drop
    }

    pub fn remove_dropped_item_from_map(&self, map_instance_state: &mut MapInstanceState, dropped_item_id: u32) {
        if let Some(dropped_item) = map_instance_state.remove_dropped_item(dropped_item_id) {
            let mut packet_zc_item_disappear = PacketZcItemDisappear::default();
            packet_zc_item_disappear.set_itaid(dropped_item_id);
            packet_zc_item_disappear.fill_raw();
            self.client_notification_sender.send(Notification::Area(
                AreaNotification::new(map_instance_state.key().map_name().clone(), map_instance_state.key().map_instance(),
                                      AreaNotificationRangeType::Fov { x: dropped_item.x(), y: dropped_item.y(), exclude_id: None },
                                      packet_zc_item_disappear.raw))).expect("Fail to send client notification");
        }
        self.server_task_queue.add_to_first_index(GameEvent::MapNotifyItemRemoved(dropped_item_id));
    }
}