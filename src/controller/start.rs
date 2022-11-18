extern crate core;

use std::fs;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::collections::HashMap;
use serde_yaml;
use serde_derive::Deserialize;
use crate::controller::containers::container_interface;
use crate::structures::config::*;

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