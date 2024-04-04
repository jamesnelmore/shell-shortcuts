use regex::{Regex, RegexBuilder};
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;

use crate::alias::Alias;

mod tests;

#[derive(Debug, PartialEq)]
pub struct AliasList {
    pub aliases: Vec<Alias>
}

impl AliasList {
    #[allow(unused)]
    pub fn new() -> AliasList {
        AliasList { aliases: vec![] }
    }

    #[allow(unused)]
    pub fn add_alias(&mut self, alias: Alias) -> Option<()> {
        if self
            .aliases
            .iter()
            .any(|a| a.shortcut() == alias.shortcut())
        {
            return None;
        }
        self.aliases.push(alias);

        Some(())
    }

    const REGEX_STRING: &'static str = concat!(
        r"^(?:alias )",            // Lookbehind matching "alias " at start of line
        r"(?<shortcut>\S+)",       // Matches text after "alias " under "="
        r#"(?: ?= ?")"#,           // Matches but does not capture "="
        r#"(?<command>.+)(?:")$"#, // Matches all text between quotes and end of line
    );

    fn get_regex() -> Regex {
        #[allow(clippy::unwrap_used)]
        RegexBuilder::new(AliasList::REGEX_STRING)
            .multi_line(true)
            .build()
            .unwrap()
    }

    pub fn aliases_from_buf(buf: &str) -> AliasList {
        let aliases = Self::get_regex()
            .captures_iter(buf)
            .map(|capture| {
                let shortcut = &capture["shortcut"].to_string();
                let command = &capture["command"].to_string();
                println!("Captured: {shortcut} | {command}");
                Alias::new(shortcut, command)
            })
            .collect::<Vec<Alias>>();

        AliasList { aliases }
    }

    pub fn iter(&self) -> (impl Iterator<Item = &Alias> + '_) {
        self.aliases.iter()
    }
}

impl Display for AliasList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = self
            .aliases
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", display + "\n")
    }
}
