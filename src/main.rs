mod alias;
mod aliases;
mod interface;

use clap::Parser;
use interface::{Commands, Interface};
use std::path::PathBuf;

// Plan
// Initialize AliasList
// - Check environment variables for path to alias file
// - Parse alias file for AliasList
// Execute user command with given AliasList

fn main() {
    let interface = Interface::parse();
    match &interface.command {
        Commands::Add { shortcut, command } => {
            todo!("Add {shortcut}=\"{command}\"")
        }
        Commands::Remove { .. } => {
            todo!("implement removing aliases")
        }
        Commands::Replace {
            old_shortcut,
            new_shortcut,
        } => {
            todo!("implement replacement")
        }
        Commands::List => {
            todo!("implement listing aliases")
        }
    };
}

fn alias_path() -> PathBuf {
    // TODO check environment variable for aliases
    ["~", ".aliases"].iter().collect()
}
