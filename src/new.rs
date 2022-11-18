use std::{env, io, str, string, path, fs};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_yaml;
use serde_derive::Deserialize;

use crate::common_commands::get_input;
use crate::structures::config::*;

pub fn generate(path: &Path, force: bool, silent: bool, overwrite: bool) -> bool {

    if !path.is_dir() { println!("Please give destination directory"); return false; }
    let project_directory: &str = path.components().last().unwrap().as_os_str().to_str().unwrap();

    if path.join("main.ro").exists() { println!("File already exists")}
    let mut name = get_input("Project Name", project_directory);
    let mut version = get_input("Version", project_directory);
    let mut description = get_input("Description", "");
    let mut authors: Vec<String> = get_input("Author", "").split_whitespace().map(str::to_string).collect();

    // Project Information
    let mut looping: bool = true;
    while looping {
        name = get_input("Project Name", project_directory);
        version = get_input("Version", project_directory);
        description = get_input("Description", "");
        authors = get_input("Author", "").split_whitespace().map(str::to_string).collect();
        println!
        ("\n\
        Project Name: {}\n\
        Project Version: {}\n\
        Authors: {}\n\
        Description: {}",
         name,
         version,
         authors.join(" "),
         description
        );
        match get_input("Confirm?", "Y").as_str() {
            "Y" | "y" => { looping = false },
            _ => {}
        }
    }
    let mut runtime = get_input("Runtime", "deepcore/rosetta_runtime");
    let mut controller = get_input("Controller", "deepcore/rosetta_controller");
    let mut max_workers: usize = get_input("Max Workers", "4").parse().unwrap();
    let mut containers: Vec<String> = get_input("Containers", "").split_whitespace().map(str::to_string).collect();

    // Rosetta Configuration
    let mut looping: bool = true;
    while looping {
        runtime = get_input("Runtime", "deepcore/rosetta_runtime");
        controller = get_input("Controller", "deepcore/rosetta_controller");
        max_workers = get_input("Max Workers", "4").parse().unwrap();
        containers = get_input("Containers", "").split_whitespace().map(str::to_string).collect();
        println!("\n\
        Runtime: {}\n\
        Controller: {}\n\
        Max Workers: {}\n\
        Containers: {}",
                 runtime,
                 controller,
                 max_workers,
                 containers.join(" ")
        );
        match get_input("Confirm?", "Y").as_str() {
            "Y" | "y" => { looping = false },
            _ => {}
        }
    }

    //RosettaConfig
    let configuration: Config = Config {
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