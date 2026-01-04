use std::fs;

pub(crate) fn init() {
    fs::create_dir(".git").expect("Failed to create .git");
    fs::create_dir(".git/objects").expect("Failed to create .git/objects");
    fs::create_dir(".git/refs").expect("Failed to create .git/refs");
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Failed to write .git/HEAD");
    println!("Initialized git directory")
}
