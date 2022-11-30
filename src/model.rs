use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub enabled: bool,
    pub obs_password: String,
    pub mappings: Vec<Mapping>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapping {
    pub monitor_name: String,
    pub obs_scene_name: String,
}