extern crate core;

use std::fs;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::collections::HashMap;
use serde_yaml;
use serde_derive::Deserialize;
use crate::controller::containers::container_interface;


fn main() {
    let filename = Path::new("config.yaml");
    container_interface::spawn("rosetta");

    println!("This is working");
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file {}", filename.to_str().unwrap());
            exit(1);
        }
    };
    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not load configuration file from {}", filename.to_str().unwrap());
            exit(1);
        }
    };
    println!("{:?}", config);
}

#[derive(Deserialize, Debug)]
struct Config {
    information: Information,
    rosetta_config: RosettaConfig,
    projects: HashMap<String, Project>,
}
#[derive(Deserialize, Debug)]
struct Information {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>
}
#[derive(Deserialize, Debug)]
struct RosettaConfig {
    runtime: String,
    controller: String,
    max_workers: usize,
    containers: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Project {
    container: String,
    entry: Option<bool>
}
