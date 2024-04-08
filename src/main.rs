mod alias;
mod alias_list;
mod error;
mod interface;

use alias::Alias;
use alias_list::AliasList;
use clap::Parser;
pub use error::Error;
use interface::Interface;
use std::path::PathBuf;
// Plan
// Initialize AliasList
// - Check environment variables for path to alias file
// - Parse alias file for AliasList
// Execute user command with given AliasList

fn main() -> Result<(), Error> {
    let aliases: AliasList = AliasList::try_from(alias_path())?;
    let interface: Interface = Interface::parse();
    interface.switchboard(aliases)
    // TODO aliases should be scannable with single quotes
}

fn alias_path() -> PathBuf {
    // TODO check environment variable for aliases
    PathBuf::from("/Users/jelmore/.aliases")
}
