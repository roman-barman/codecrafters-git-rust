mod args;
mod blob_object;
mod blob_storage;
mod commands;

use crate::args::{AppArgs, Command};
use crate::blob_storage::BlobStorage;
use clap::Parser;

fn main() {
    let command = AppArgs::parse().command;
    let mut storage = BlobStorage::default();

    let result = match command {
        Command::Init => commands::init(&storage).map_err(anyhow::Error::new),
        Command::CatFile(args) => {
            commands::cat_file(&mut storage, &args.hash).map_err(anyhow::Error::new)
        }
        Command::HashObject(args) => {
            commands::hash_object(&mut storage, &args.path, args.write).map_err(anyhow::Error::new)
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
