extern crate core;

mod new;
mod packages;
mod common_commands;

pub mod controller;
pub mod structures;

use std::path::Path;
use clap::{arg, Parser, Subcommand};
use reqwest::Client;

#[derive(Parser,Debug)]
#[clap(author="quexeky", version, about="Rosetta argument parser")]
struct Arguments {
    /*#[clap(default_value="./",short, long)]
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,*/
    #[clap(subcommand)]
    cmd: SubCommand,

}
#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Create New Rosetta project
    New {
        #[clap(short, long, takes_value=false)]
        /// Does not prompt, takes safest actions
        silent: bool,
        #[clap(short, long, takes_value=false)]
        /// Allows unsafe operations. Use with --silent to run all operations
        force: bool,
        #[clap(short, long, takes_value=false)]
        /// Allows project creation in non-empty directories
        overwrite: bool,

        #[clap(default_value="./", short, long)]
        /// Change target directory
        destination: String,

    },
    /// Rosetta package management
    Package {
        #[clap(short, long, multiple_values=true, value_delimiter=':', forbid_empty_values = true, validator=validate_package_name)]
        /// Install packages and modules
        install: Vec<String>,
        #[clap(short, long, multiple_values=true, value_delimiter=':', forbid_empty_values = true, validator=validate_package_name)]
        /// Remove packages and modules
        remove: Vec<String>,
        #[clap(short = 'R', long, multiple_values=true, value_delimiter=':', forbid_empty_values = true, validator=validate_package_name)]
        /// Reinstall packages and modules
        reinstall: Vec<String>,
        #[clap(default_value="./", short, long)]
        /// Change target directory
        destination: String,

    },
}


fn validate_package_name(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "package name cannot have leading and trailing space",
        ))
    } else {
        Ok(())
    }
}

async fn main() {
    /*
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::New { silent, force, destination, overwrite} => {
            new::generate(Path::new(&destination), force, silent, overwrite);
        }
        SubCommand::Package { install, remove, reinstall, destination} => {
            println!("install: {}, remove: {}, reinstall: {}", install.join(":"), remove.join(":"), reinstall.join(":"));
        }
    }*/
    let client = Client::new();
    packages::download_file(&client,"https://cloud.decduck3.com/index.php/avatar/admin/64", "/home/quexeky/CLionProjects/rosettacli/RosettaProject").await?;
}
