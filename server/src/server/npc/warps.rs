
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use crate::server::core::configuration::Config;
use crate::server::core::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::npc::{Npc, NpcLoader};

static PARALLEL_EXECUTIONS: usize = 100; // TODO add a conf for this
static WARP_CONF_PATH: &str = "./npc/scripts_warps.conf";

#[derive(SettersAll, Clone, Debug)]
pub struct Warp {
    pub map_name: String,
    pub name: String,
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub x_size: u16,
    pub y_size: u16,
    pub dest_map_name: String,
    pub to_x: u16,
    pub to_y: u16,
}

impl Npc for Warp {
    fn parse_npc(file: &File, _config: &'static Config) -> Result<Vec<Warp>, String> {
        let reader = BufReader::new(file);
        let mut warps = Vec::<Warp>::new();
        for line in reader.lines() {
            let mut warp = Warp::new();
            if line.is_err() {
                break;
            }
            let line = line.unwrap();
            if line.starts_with("//") || !line.contains("\twarp\t") {
                continue;
            }
            // A warp "npc" definition is as below
            // anthell01,253,32,0	warp	ant01	2,1,anthell02,34,263
            let line_fragment = line.split('\t').collect::<Vec<&str>>();
            let source_information = line_fragment[0];
            let name = line_fragment[2];
            let warp_and_destination_information = line_fragment[3];
            let source_information_split = source_information.split(',').collect::<Vec<&str>>();
            let warp_and_destination_information_split = warp_and_destination_information.split(',').collect::<Vec<&str>>();
            warp.set_name(name.to_string());
            warp.set_map_name(source_information_split[0].to_string());
            warp.set_x(source_information_split[1].parse::<u16>().unwrap());
            warp.set_y(source_information_split[2].parse::<u16>().unwrap());
            warp.set_x_size(warp_and_destination_information_split[0].parse::<u16>().unwrap());
            warp.set_y_size(warp_and_destination_information_split[1].parse::<u16>().unwrap());
            warp.set_dest_map_name(warp_and_destination_information_split[2].to_string());
            warp.set_to_x(warp_and_destination_information_split[3].parse::<u16>().unwrap());
            warp.set_to_y(warp_and_destination_information_split[4].parse::<u16>().unwrap());
            warps.push(warp);
        }
        Ok(warps)
    }

    fn get_map_name(&self) -> String {
        self.map_name.clone()
    }
}

impl Warp {
    pub fn new() -> Warp {
        Warp {
            id: 0,
            name: "".to_string(),
            map_name: "".to_string(),
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            dest_map_name: "".to_string(),
            to_x: 0,
            to_y: 0
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }


    pub async fn load_warps(config: &'static Config) -> HashMap<String, Vec<Warp>> {
        let start = Instant::now();
        let npc_loader = NpcLoader {
            conf_file: File::open(Path::new(WARP_CONF_PATH)).unwrap(),
            parallel_execution: PARALLEL_EXECUTIONS,
        };
        let warps = npc_loader.load_npc::<Warp>(config).await;
        info!("load {} warps in {} secs", warps.iter().fold(0, |memo, curr| memo + curr.1.len()), start.elapsed().as_millis() as f32 / 1000.0);
        warps
    }
}

impl ToMapItem for Warp {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, 45, MapItemType::Warp)
    }
}