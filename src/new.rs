use std::{env, io, str, string, path, fs};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use crate::new::common_commands::get_input;


pub fn generate(path: &Path, force: bool, silent: bool, overwrite: bool) -> bool {

    if !path.is_dir() { println!("Please give destination directory"); return false; }
    let project_directory: &str = path.components().last().unwrap().as_os_str().to_str().unwrap();

    if path.join("main.ro").exists() { println!("File already exists")}

    let project_name: String = get_input("Project Name", project_directory);
    if project_name.is_ascii() { println!("Project name must use only alphanumeric characters"); return false; }
    let project_version: String = get_input("Version", project_directory);
    let project_author: String = get_input("Author", "");
    let project_description: String = get_input("Description", "");

    println!
    ("\n\
    Project Name: {}\n\
    Project Version: {}\n\
    Author: {}\n\
    Description: {}",
     project_name,
     project_version,
     project_author,
     project_description
    );







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
    use std::{cmp, io, str};
    use std::io::{stdout, Write};
    use std::process::exit;
    extern crate termsize;

    pub fn get_input(prompt: &str, default: &str) -> String{
        let termsize::Size {rows, cols} = termsize::get().unwrap();
        print!("{} [{}]: ",prompt, default);
        stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) { Ok(..) => {}, Err(e) => {}, }
        if input.trim() == "" {
            stdout().flush().unwrap();
            return default.to_string()
        }
        stdout().flush().unwrap();
        input.trim().to_string()
    }
}