#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").expect("Failed to create .git");
        fs::create_dir(".git/objects").expect("Failed to create .git/objects");
        fs::create_dir(".git/refs").expect("Failed to create .git/refs");
        fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Failed to write .git/HEAD");
        println!("Initialized git directory")
    } else {
        println!("unknown command: {}", args[1])
    }
}
