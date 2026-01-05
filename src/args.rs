use clap::{Args, Parser, Subcommand};

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
}

#[derive(Args, Debug, Clone)]
pub(crate) struct CatFileArgs {
    pub(crate) hash: String,
    #[clap(short, long)]
    pub(crate) pretty: bool,
}
