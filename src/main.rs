mod commands;

use crate::commands::init;
use std::env;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        init()
    } else {
        println!("unknown command: {}", args[1])
    }
}
