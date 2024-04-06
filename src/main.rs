mod alias;
mod aliases;
mod interface;

use alias::Alias;
use aliases::{AliasFile, AliasList};
use clap::Parser;
use interface::{Commands, Interface};
use std::error::Error;
use std::path::PathBuf;
// Plan
// Initialize AliasList
// - Check environment variables for path to alias file
// - Parse alias file for AliasList
// Execute user command with given AliasList

fn main() -> Result<(), Box<dyn Error>> {
    let mut alias_file = AliasFile::new(alias_path())?;
    let interface = Interface::parse();
    match &interface.command {
        Commands::Add { shortcut, command } => {
            let alias: Alias = Alias::new(shortcut, command).or_else(|_| {
                println!("Invalid input");
                todo!("Graceful error handling for invalid aliases");
            })?;
            alias_file.aliases().add_alias(alias);
            todo!("Save file")
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
            let display = alias_file.aliases().to_string();
            println!("{display}");
        }
    };

    Ok(())
}

fn alias_path() -> PathBuf {
    // TODO check environment variable for aliases
    ["~", ".aliases"].iter().collect()
}
