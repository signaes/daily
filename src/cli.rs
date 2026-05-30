use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        date: Option<String>,
        content: String,
    },
    Remove {
        date: Option<String>,
        id: u32,
    },
    Edit {
        date: Option<String>,
        id: u32,
        content: String,
    },
    Delete {
        date: Option<String>,
        id: u32,
    },
}

#[derive(Debug, Parser)]
#[command(name = "daily", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'd', long, help = "Path to the SQLite database file")]
    pub db: Option<PathBuf>,
}
