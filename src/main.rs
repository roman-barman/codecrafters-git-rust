mod args;
mod commands;

use crate::args::{AppArgs, Command};
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    let command = AppArgs::parse().command;
    match command {
        Command::Init => commands::init()?,
        Command::CatFile(args) => commands::cat_file(&args.hash),
    };

    Ok(())
}
