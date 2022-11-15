use std::io::{BufReader, BufRead };
use std::os::unix::net::{UnixListener, UnixStream };
use std::{fs, thread};
use std::path::Path;
use std::process::exit;
use crate::controller::containers::parse::input;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        if line.as_ref().unwrap() == "quit" {
            break;
        }
        input::parse_line(line.as_ref().unwrap());
    }
}

pub fn spawn(location: &str) {
    let socket: String = format!("/tmp/{}.sock", location);
    if Path::new(&socket).exists() {
        match fs::remove_file(&socket) {
            Ok(()) => {},
            Err(err) => { return; }
        }

    }
    let listener = UnixListener::bind(&socket).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}