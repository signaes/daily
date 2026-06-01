mod date;

mod config;
use config::Config;

mod cli;
use cli::{Cli, Commands};

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = Config::load(cli.db, cli.context)?;

    let db = &config.db;
    let context = &config.context;

    match cli.command {
        Commands::Add { date, content } => {
            todo!("Add entry to {db:?}: context={context:?} date={date:?}, content={content:?}")
        }
        Commands::Remove { date, id } => {
            todo!("Remove entry from {db:?}: context={context:?} date={date:?}, id={id}")
        }
        Commands::Edit { date, id, content } => {
            todo!(
                "Edit entry in {db:?}: context={context:?} date={date:?}, id={id}, content={content:?}"
            )
        }
    }
}
