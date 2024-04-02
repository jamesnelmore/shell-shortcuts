// Rustc lints
#![forbid(unsafe_code)]
// Clippy lints
#![warn(clippy::pedantic)]
#![deny(
    clippy::enum_glob_use, /*clippy::unwrap_used*/)]

mod alias;
mod aliases;
mod interface;

use interface::{Interface, Commands};
use clap::Parser;

// Plan
// Initialize AliasList
// - Check environment variables for path to alias file
// - Parse alias file for AliasList
// Execute user command with given AliasList


fn main() {
    let interface = Interface::parse();
    match &interface.command {
        Commands::Add { .. } => {
            todo!("implement adding aliases")
        }
        Commands::Remove { .. }=> {
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
