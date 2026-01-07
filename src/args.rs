use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Git copy CLI
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct AppArgs {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum Command {
    /// Initialize a git repository
    Init,
    /// Print the contents of an object
    #[command(name = "cat-file")]
    CatFile(CatFileArgs),
    /// Hash an object
    #[command(name = "hash-object")]
    HashObject(HashObjectArgs),
}

#[derive(Args, Debug, Clone)]
pub(crate) struct CatFileArgs {
    /// The hash of the object to print
    pub(crate) hash: String,
    /// Print the object in a human readable format
    #[clap(short, long)]
    pub(crate) pretty: bool,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct HashObjectArgs {
    /// The file to hash
    pub(crate) path: PathBuf,
    /// Write the file to git objects store
    #[clap(short, long)]
    pub(crate) write: bool,
}
