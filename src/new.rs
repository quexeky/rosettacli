use std::{env, io, str, string, path, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;
use colored::Colorize;
use serde::{Serialize, Deserialize};


pub fn generate(path: &Path, force: bool, silent: bool, overwrite: bool) -> bool {

    if !path.is_dir() { println!("Please give destination directory, not a file"); return false; }

    if fs::read_dir(path).unwrap().count() != 0 {
        if overwrite || force {
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                let file_check: bool = file.file_name().into_string().unwrap().ends_with(".ro");
                if file_check && !force {
                    println!("Project already exists in folder. Use --force to enable overwriting projects");
                    return false;
                } else if file_check && silent { fs::remove_file(&path).expect("Could not remove project files. Aborting"); } else if file_check && force {
                    let input = common_commands::y_n_getter("Project already exists in directory. Delete old project and continue?", "n");
                    match input.to_lowercase().trim() {
                        "y" => {
                            match fs::remove_file(&path) {
                                Ok(..) => {}
                                Err(e) => { println!("{}", e) }
                            }
                        },
                        _ => {
                            println!("Exiting");
                            return false;
                        }
                    }
                } else if !file_check {
                    if !silent {
                        let input = common_commands::y_n_getter("Directory not empty. Continue?", "n");
                        match input.as_str() {
                            "y" => {},
                            _ => {
                                println!("Exiting");
                                return false;
                            }
                        }
                    } else if silent { println!("Continuing in non-empty directory") }
                }

            }
            for file in fs::read_dir(path) {
                if file.file_name().into_string().unwrap() == "config.json" {
                    let input = common_commands::get_input("Configuration file already exists. Use old config file? ", "n");
                    match input.as_str() {
                        "y" => {
                            new_config_file(path.to_path_buf());
                            continue;
                        }
                        _ => {
                            fs::remove_file(file.path()).expect("Could not delete file. Do you have permission to do that?");
                            File::create(file.path().join("config.json")).expect("Could not create file. Do you have permission to do that?");
                            fs::write(file.path().join("config.json"), "").expect("Could not write file. Do you have permission to do that?");
                            exit(0);
                        }
                    }
                } else { new_config_file(path.to_path_buf()); }
            }
            fs::write(path.join("main.ro"), "").expect("Could not create new project. Do you have permission to do that?");
            println!("Created file main.ro");

            println!("Created new project in {:?}", path);
        }
        else { println!("Files already exist in directory. Use --overwrite to enable overwriting in directories"); return false }
    }


    return true
}

fn new_config_file(path: PathBuf) {
    println!("new config working");
    let project_name = common_commands::get_input("Project name? {}", path.file_name().unwrap().to_str().unwrap());
}

struct Config {
    id: String,
    entry: String,
    project_name: String,
    modules: Vec<String>,
    clients: Vec<String>,
}

mod common_commands {
    use std::io;
    use std::io::{stdout, Write};
    use std::process::exit;

    pub fn get_input(prompt: &str, default: &str) -> String{
        print!("{} ({})",prompt, default);
        match stdout().flush() { Ok(..) => {}, Err(..) =>  { panic!() }}
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        if input.trim() == "" { return default.to_string() }
        input.trim().to_string()
    }
    pub fn y_n_getter(prompt: &str, default: &str) -> String{
        print!("{} ({}) ",prompt, default);
        match stdout().flush() { Ok(..) => {}, Err(..) =>  { exit(-1) }}
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(..) => {},
            Err(e) => {
                println!("{}", e);
                exit(-1)
            },
        }
        match input.trim().to_lowercase().as_str() {
            "" => {return default.to_string();}
            "y" | "yes" => {return "y".to_string()}
            _ => {return "n".to_string()}
        }
    }
}