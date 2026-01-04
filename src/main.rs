mod commands;

use std::env;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => commands::init(),
        "cat-file" => {
            let hash = &args[2];
            commands::cat_file(hash);
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
