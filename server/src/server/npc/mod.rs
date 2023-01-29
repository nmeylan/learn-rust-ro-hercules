use futures::future::join_all;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use crate::server::core::configuration::Config;

pub mod mob_spawn;
pub mod script;
pub mod warps;

pub struct NpcLoader {
    pub(crate) conf_file: File,
    pub(crate) parallel_execution: usize,
}

pub trait Npc {
    fn parse_npc(file: &File, config: &'static Config) -> Result<Vec<Self>, String>
    where
        Self: Sized;
    fn get_map_name(&self) -> String;
}

impl NpcLoader {
    pub async fn load_npc<T: Npc + Clone + Send + 'static>(&self, config: &'static Config) -> HashMap<String, Vec<T>> {
        let semaphore = Semaphore::new(self.parallel_execution);
        let reader = BufReader::new(&self.conf_file);
        let npcs_by_map = Arc::new(Mutex::new(HashMap::<String, Vec<T>>::new()));
        let mut futures: Vec<JoinHandle<()>> = Vec::new();
        for line in reader.lines() {
            if line.is_err() {
                break;
            }
            let mut line = line.unwrap();
            if !line.starts_with("npc:") {
                continue;
            }
            line = line.replace("npc: ", "");
            let npc_script_path = line.trim().to_string();
            let _ = semaphore.acquire().await.unwrap();
            let res = npcs_by_map.clone();
            futures.push(tokio::task::spawn_blocking(move || {
                let npc_script_file_res = File::open(Path::new(&npc_script_path));
                if npc_script_file_res.is_err() {
                    warn!(
                        "Not able to load npc script: {}, due to {}",
                        npc_script_path,
                        npc_script_file_res.err().unwrap()
                    );
                    return;
                }

                let npcs_result = T::parse_npc(&npc_script_file_res.unwrap(), config);
                if npcs_result.is_err() {
                    error!("{}", npcs_result.err().unwrap());
                    return;
                }
                let npcs = npcs_result.unwrap();
                for npc in npcs {
                    let mut res_guard = res.lock().unwrap();
                    let map_name = npc.get_map_name();
                    let entry = res_guard.entry(map_name).or_insert(Default::default());
                    entry.push(npc);
                }
            }));
        }
        join_all(futures).await;
        let guard = npcs_by_map.lock().unwrap();
        let mut res = HashMap::<String, Vec<T>>::new();
        guard.iter().for_each(|(k, v)| {
            res.insert(k.clone(), v.clone());
        });
        res
    }
}
