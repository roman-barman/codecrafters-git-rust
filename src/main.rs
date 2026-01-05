mod args;
mod commands;

use crate::args::{AppArgs, Command};
use clap::Parser;

fn main() {
    let command = AppArgs::parse().command;
    let result = match command {
        Command::Init => commands::init().map_err(anyhow::Error::new),
        Command::CatFile(args) => commands::cat_file(&args.hash).map_err(anyhow::Error::new),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
