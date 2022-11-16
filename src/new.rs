use std::{env, io, str, string, path, fs};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_yaml;
use serde_derive::Deserialize;
use toml::Value::String;

use crate::common_commands::get_input;
use crate::structures::config::*;

pub fn generate(path: &Path, force: bool, silent: bool, overwrite: bool) -> bool {

    if !path.is_dir() { println!("Please give destination directory"); return false; }
    let project_directory: &str = path.components().last().unwrap().as_os_str().to_str().unwrap();

    if path.join("main.ro").exists() { println!("File already exists")}



    //Information
    let name = get_input("Project Name", project_directory);
    let version = get_input("Version", project_directory);
    let authors: Vec<String> = get_input("Author", "").split_whitespace().map(str::to_string).collect();
    let description = get_input("Description", "");

    //RosettaConfig
    let runtime = get_input("Runtime", "deepcore/rosetta_runtime");
    let controller = get_input("Controller", "deepcore/rosetta_controller");
    let max_workers: usize = get_input("Max Workers", "4").parse().unwrap();
    let containers: Vec<String> = get_input("Containers", "").split_whitespace().map(str::to_string).collect();


    println!
    ("\n\
    Project Name: {}\n\
    Project Version: {}\n\
    Authors: {:?}\n\
    Description: {}",
     name,
     version,
     authors,
     description
    );
    let mut configuration: Config = Config {
        information: Information {
            name,
            version,
            description,
            authors,
        },
        rosetta_config: RosettaConfig {
            runtime,
            controller,
            max_workers,
            containers
        },
        projects: HashMap::new()
    };
    return true
}

fn new_config_file(path: PathBuf) {
    println!("new config working");
    let project_name = get_input("Project name? {}", path.file_name().unwrap().to_str().unwrap());
}

