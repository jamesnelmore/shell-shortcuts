// Rustc lints
#![forbid(unsafe_code)]
#![allow(dead_code)]
// Clippy lints
#![warn(clippy::pedantic)]
#![deny(
    clippy::enum_glob_use, /*clippy::unwrap_used*/)]

mod alias;
mod aliases;
mod interface;

use aliases::AliasList;
use clap::Parser;
use interface::{Commands, Interface};
use std::{path, path::PathBuf, str::FromStr};

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
        Commands::Replace => {
            todo!("implement replacement")
        }
        Commands::List => {
            todo!("implement listing aliases")
        }
    };
}

fn path() -> PathBuf {
    // TODO check settings 
    ["~", ".aliases"].iter().collect()
}
