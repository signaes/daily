mod config;
use config::Config;

mod cli;
use cli::{Cli, Commands};

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let mut config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    // CLI --db flag overrides the config file
    if let Some(cli_db_path) = cli.db {
        config.db = Some(cli_db_path)
    }

    let db_path = match config.db {
        Some(p) => p,
        None => {
            eprintln!("Error: Missing database path.");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Add { date, content } => {
            todo!("Add entry to {db_path:?}: date={date:?}, content={content:?}")
        }
        Commands::Remove { date, id } => {
            todo!("Remove entry from {db_path:?}: date={date:?}, id={id}")
        }
        Commands::Edit { date, id, content } => {
            todo!("Edit entry in {db_path:?}: date={date:?}, id={id}, content={content:?}")
        }
        Commands::Delete { date, id } => {
            todo!("Delete entry from {db_path:?}: date={date:?}, id={id}")
        }
    }
}
