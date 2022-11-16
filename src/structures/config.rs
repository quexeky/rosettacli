use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_yaml;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub information: Information,
    pub rosetta_config: RosettaConfig,
    pub projects: HashMap<String, Project>,
}
#[derive(Deserialize, Debug)]
pub struct Information {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>
}
#[derive(Deserialize, Debug)]
pub struct RosettaConfig {
    pub runtime: String,
    pub controller: String,
    pub max_workers: usize,
    pub containers: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub container: String,
    pub entry: Option<bool>
}

