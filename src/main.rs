mod alias_list;
mod alias;
mod interface;

use alias::Alias;
use alias_list::AliasList;
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
    let mut aliases: AliasList = AliasList::try_from(alias_path())?;
    let interface = Interface::parse();
    match &interface.command {
        Commands::Add { shortcut, command } => {
            #[allow(clippy::bind_instead_of_map)] // TODO fix error
            let alias: Alias = Alias::new(shortcut, command).or_else(|_| -> Result<Alias, Box<dyn Error>> {
                println!("Invalid input");
                todo!("Graceful error handling for invalid aliases");
            })?;
            aliases.add_alias(alias);
            todo!("Save file")
        }
        Commands::Remove { .. } => {
            todo!("implement removing aliases")
        }

        #[allow(unused_variables)] // Only until implemented
        Commands::Replace {
            old_shortcut,
            new_shortcut,
        } => {
            todo!("implement replacement")
        }
        Commands::List => {
            let display = aliases.to_string();
            println!("List: {display}");
        }
    }; // TODO aliases should be scannable with single quotes

    Ok(())
}

fn alias_path() -> PathBuf {
    // TODO check environment variable for aliases
    PathBuf::from("/Users/jelmore/.aliases")
}
