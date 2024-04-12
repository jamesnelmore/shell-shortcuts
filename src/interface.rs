use crate::alias_path;
use crate::{Alias, AliasList, Error};
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Interface {
    #[command(subcommand)]
    pub command: Commands,
}

#[non_exhaustive]
#[derive(Subcommand)]
pub enum Commands {
    Add { shortcut: String, command: String },
    Remove { old_shortcut: String },
    Replace { old: String, new: String },
    List,
}

impl Interface {
    pub fn switchboard(&self, mut aliases: AliasList) -> Result<(), Error> {
        let path = alias_path();
        match &self.command {
            Commands::Add { shortcut, command } => {
                let new_alias = Alias::new(shortcut, command)?;
                aliases.add_alias(new_alias);
            }
            Commands::Remove { old_shortcut } => aliases.remove_alias_by_shortcut(old_shortcut)?,
            Commands::Replace { old, new } => aliases.replace_shortcut(old, new.to_string())?,
            Commands::List => {
                let display = aliases.to_string();
                println!("{display}");
                return Ok(());
            }
        }
        aliases.save_to_file(path)?;
        update_shell()
    }
}

fn update_shell() -> Result<(), Error> {
    let command = Command::new("exec zsh").output().expect("Debug");
    // .map(|_| ())
    // .map_err(crate::Error::IOError);
    println!("Ran command");
    Ok(())
}
