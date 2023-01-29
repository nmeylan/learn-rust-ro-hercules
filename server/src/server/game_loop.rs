use std::borrow::Borrow;
use std::sync::{Arc};
use std::sync::mpsc::SyncSender;
use std::thread::{sleep};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::runtime::Runtime;

use packets::packets::{Packet, PacketZcNotifyPlayermove};
use crate::PersistenceEvent;
use crate::PersistenceEvent::SaveCharacterPosition;
use crate::server::core::movement::{Movable, Movement};
use crate::server::core::path::{manhattan_distance, path_search_client_side_algorithm};
use crate::server::core::position::Position;
use crate::server::events::game_event::{CharacterMovement, GameEvent};

use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::map_event::{MapEvent};
use crate::server::events::persistence_event::{SavePositionUpdate};

use crate::server::core::map_item::{MapItemType, ToMapItemSnapshot, ToMapItem};

use crate::server::Server;
use crate::server::service::battle_service::BattleService;
use crate::server::service::character::character_service::{CharacterService};
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::item_service::{ItemService};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::server_service::ServerService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;


const MOVEMENT_TICK_RATE: u128 = 20;
const GAME_TICK_RATE: u128 = 40;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, runtime: Runtime) {
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut server_state_mut = server_ref.state_mut();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        GameEvent::CharacterChangeMap(event) => {
                            let map_instance = server_ref.state().get_map_instance(&event.new_map_name, event.new_instance_id)
                                .unwrap_or_else(|| ServerService::instance().create_map_instance(server_state_mut.as_mut(), GlobalConfigService::instance().get_map(&event.new_map_name), event.new_instance_id));
                            let character = server_state_mut.characters_mut().get_mut(&event.char_id).unwrap();
                            CharacterService::instance().change_map(map_instance.state().borrow().as_ref(), &event, character);
                            let char_map_item = character.to_map_item();
                            map_instance.add_to_next_tick(MapEvent::InsertCharToMap(char_map_item));
                            server_ref.add_to_next_tick(GameEvent::CharacterInitInventory(character.char_id));
                            let account_id = character.account_id;
                            server_state_mut.insert_map_item(account_id, char_map_item);
                        }
                        GameEvent::CharacterRemoveFromMap(character_remove_from_map) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_remove_from_map.char_id).unwrap();
                            character.movements = vec![];
                            if let Some(instance) = server_ref.state().get_map_instance(&character_remove_from_map.map_name, character_remove_from_map.instance_id) {
                                instance.add_to_next_tick(MapEvent::RemoveCharFromMap(character_remove_from_map.char_id));
                            }
                        }
                        GameEvent::CharacterClearFov(char_id) => {
                            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                            character.clear_map_view();
                        }
                        GameEvent::CharacterLoadedFromClientSide(char_id) => {
                            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                            character.loaded_from_client_side = true;
                            character.clear_map_view();
                        }
                        GameEvent::CharacterMove(_) => {
                            // handled by dedicated thread
                        }
                        GameEvent::CharacterUpdateLook(character_look) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_look.char_id).unwrap();
                            CharacterService::instance().change_look(character_look, character)
                        }
                        GameEvent::CharacterUpdateZeny(zeny_update) => {
                            let character = server_state_mut.characters_mut().get_mut(&zeny_update.char_id).unwrap();
                            CharacterService::instance().update_zeny(&runtime, zeny_update, character);
                        }
                        GameEvent::CharacterAddItems(add_items) => {
                            let character = server_state_mut.characters_mut().get_mut(&add_items.char_id).unwrap();
                            InventoryService::instance().add_items_in_inventory(&runtime, add_items, character);
                        }
                        GameEvent::CharacterInitInventory(char_id) => {
                            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                            InventoryService::instance().reload_inventory(&runtime, char_id, character);
                            InventoryService::instance().reload_equipped_item_sprites(character);
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterUpdateWeight(char_id) => {
                            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                            CharacterService::instance().update_weight(character);
                        }
                        GameEvent::CharacterUseItem(character_user_item) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_user_item.char_id).unwrap();
                            ItemService::instance().use_item(server_ref.clone(), &runtime, character_user_item, character);
                        }
                        GameEvent::CharacterAttack(character_attack) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_attack.char_id).unwrap();
                            if !character.is_attacking() {
                                character.set_attack(character_attack.target_id, character_attack.repeat, 0);
                            }
                        }
                        GameEvent::CharacterEquipItem(character_equip_item) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_equip_item.char_id).unwrap();
                            let index = character_equip_item.index;
                            InventoryService::instance().equip_item(character, character_equip_item);
                            character.get_item_from_inventory(index)
                                .map(|item| InventoryService::instance().sprite_change_packet_for_item(character, item)
                                    .map(|packet| CharacterService::instance().send_area_notification_around_characters(character, packet)));
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterTakeoffEquipItem(character_takeoff_equip_item) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_takeoff_equip_item.char_id).unwrap();
                            let index = character_takeoff_equip_item.index;
                            InventoryService::instance().takeoff_equip_item(character, index);
                            character.get_item_from_inventory(index)
                                .map(|item| InventoryService::instance().sprite_change_packet_for_item(character, item)
                                    .map(|packet| CharacterService::instance().send_area_notification_around_characters(character, packet)));
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterCalculateStats(char_id) => {
                            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterChangeLevel(character_change_level) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                            let delta = CharacterService::instance().update_base_level(character, character_change_level.set_level, character_change_level.add_level);
                            if delta < 0 {
                                // TODO ensure equip required min level
                            }
                        }
                        GameEvent::CharacterChangeJobLevel(character_change_level) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                            let delta = CharacterService::instance().update_job_level(character, character_change_level.set_level, character_change_level.add_level);
                            if delta < 0 {
                                // TODO ensure equip required min level
                            }
                        }
                        GameEvent::CharacterChangeJob(character_change_job) => {
                            let character = server_state_mut.characters_mut().get_mut(&character_change_job.char_id).unwrap();
                            CharacterService::instance().change_job(character, character_change_job.job);
                            // TODO ensure equip required class
                        }
                    }
                }
            }
            for (_, character) in server_state_mut.characters_mut().iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
                let map_instance = server_ref.state().get_map_instance(character.current_map_name(), character.current_map_instance());
                if let Some(map_instance) = map_instance {
                    CharacterService::instance().load_units_in_fov(server_ref.as_ref(), character, map_instance.state().borrow().as_ref());
                    Self::handle_character_attack(&server_ref, tick, character)
                }
            }
            for (_, map) in server_ref.state().map_instances().borrow().iter() {
                for instance in map.iter() {
                    instance.add_to_next_tick(MapEvent::UpdateMobsFov(server_state_mut.characters_mut().iter().map(|(_, character)| character.to_map_item_snapshot()).collect()));
                }
            }
            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (GAME_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!("Less than 5 milliseconds of sleep, game loop is too slow - {}ms because game loop took {}ms", sleep_duration, time_spent);
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }

    fn handle_character_attack(server_ref: &Arc<Server>, tick: u128, character: &mut Character) {
        if !character.is_attacking() {
            return;
        }

        let map_item = server_ref.state().map_item(character.attack().target, character.current_map_name(), character.current_map_instance());
        if let Some(map_item) = map_item {
            let range = if let Some((_, weapon)) = character.right_hand_weapon() {
                GlobalConfigService::instance().get_item(weapon.item_id).range.unwrap_or(1) as u16
            } else {
                1
            };
            let target_position = server_ref.state().map_item_x_y(&map_item, character.current_map_name(), character.current_map_instance()).unwrap();
            let is_in_range = range as i16 >= manhattan_distance(character.x, character.y, target_position.x, target_position.y) as i16 - 1;
            let maybe_map_instance = server_ref.state().get_map_instance(character.current_map_name(), character.current_map_instance());
            let map_instance = maybe_map_instance.as_ref().unwrap();
            if !is_in_range && !character.is_moving() {
                let path = path_search_client_side_algorithm(map_instance.x_size(), map_instance.y_size(), map_instance.state().cells(), character.x, character.y, target_position.x, target_position.y);
                let path = Movement::from_path(path, tick);
                let current_position = Position { x: character.x, y: character.y, dir: 0 };
                debug!("Too far from target, moving from {} toward it: {}", current_position, target_position);
                server_ref.add_to_next_movement_tick(GameEvent::CharacterMove(CharacterMovement {
                    char_id: character.char_id,
                    start_at: tick,
                    destination: target_position,
                    current_position,
                    path,
                    cancel_attack: false,
                }));
            } else if is_in_range {
                character.clear_movement();
                let maybe_damage = BattleService::instance().attack(character, map_item, tick);
                if let Some(damage) = maybe_damage {
                    if matches!(*map_item.object_type(), MapItemType::Mob) {
                        map_instance.add_to_next_tick(MapEvent::MobDamage(damage));
                    }
                }
            }
        } else {
            character.clear_attack();
        }
    }


    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>) {
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut server_state_mut = server_ref.state_mut();
            if let Some(tasks) = server_ref.pop_movement_task() {
                for task in tasks {
                    if let GameEvent::CharacterMove(character_movement) = task {
                        let character = server_state_mut.characters_mut().get_mut(&character_movement.char_id).unwrap();
                        if character_movement.cancel_attack {
                            character.clear_attack();
                        }
                        let speed = character.status.speed;
                        let maybe_previous_movement = character.pop_movement();
                        character.set_movement(character_movement.path);
                        let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
                        if let Some(movement) = character.peek_mut_movement() {
                            if let Some(previous_movement) = maybe_previous_movement {
                                debug!("change path! was {} will {}, move at {}",previous_movement.position(), movement.position(), previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                // movement.set_move_at(previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()) + MOVEMENT_TICK_RATE);
                            } else {
                                movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()));
                                debug!("will move at {}", movement.move_at());
                            }
                            packet_zc_notify_playermove.set_move_start_time(movement.move_at() as u32); // todo: time conversion check on client side ???
                        }

                        packet_zc_notify_playermove.set_move_data(character_movement.current_position.to_move_data(&character_movement.destination));
                        packet_zc_notify_playermove.fill_raw();
                        client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_notify_playermove.raw_mut()))))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                    }
                }
            }

            // If movement not smooth:
            // teleport in front -> server movement faster than client movement
            // teleport back -> server movement slower than client movement
            let mut character_finished_to_move = vec![];
            for (_, character) in server_state_mut.characters_mut().iter_mut().filter(|(_, character)| character.is_moving()) {
                let speed = character.status.speed;
                if let Some(movement) = character.peek_movement() {
                    if tick >= movement.move_at() {
                        info!("move {} at {}", movement.position(), movement.move_at());
                        let movement = character.pop_movement().unwrap();
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.state().get_map_instance_from_character(character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.state().is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                ServerService::instance().schedule_warp_to_walkable_cell(server_ref.state_mut().as_mut(), warp.dest_map_name.as_str(), warp.to_x, warp.to_y, character.char_id);
                                character.clear_movement();
                                continue;
                            }
                        }
                        if let Some(next_movement) = character.peek_mut_movement() {
                            next_movement.set_move_at(tick + Movement::delay(speed, next_movement.is_diagonal()))
                        } else {
                            character_finished_to_move.push(character);
                        }
                    }
                }
            }
            for character in character_finished_to_move {
                persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() })).expect("Fail to send persistence notification");
            }

            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (MOVEMENT_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!("Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop took {}ms", sleep_duration, time_spent);
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }
}

