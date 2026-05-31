use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(long)]
        date: Option<String>,
        content: String,
    },
    Remove {
        #[arg(long)]
        date: Option<String>,
        id: u32,
    },
    Edit {
        #[arg(long)]
        date: Option<String>,
        id: u32,
        content: String,
    },
}

#[derive(Debug, Parser)]
#[command(name = "daily", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, help = "Path to the SQLite database file")]
    pub db: Option<PathBuf>,

    #[arg(short = 'c', long, help = "The context to use (work, personal, …)")]
    pub context: Option<String>,
}
