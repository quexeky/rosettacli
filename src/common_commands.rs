use std::{ io, str };
use std::io::{ stdout, Write };
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
