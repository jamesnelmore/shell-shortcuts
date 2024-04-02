// Rustc lints
#![forbid(unsafe_code)]

// Clippy lints
#![warn(clippy::pedantic)]
#![deny(
    clippy::enum_glob_use, /*clippy::unwrap_used*/)]

mod alias;
mod aliases;
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Interface {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { shortcut: String, command: String },
    Remove,
    Replace,
    List,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let interface = Interface::parse();
    match &interface.command {
        Commands::Add {..} => {
            todo!("implement adding aliases")
        }
        Commands::Remove => {
            todo!("implement removing aliases")
        }
        Commands::Replace => {
            todo!("implement replacement")
        }
        Commands::List => {
            todo!("implement listing aliases")
        }
    }
}
