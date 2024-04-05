use regex::{Regex, RegexBuilder};




use crate::alias::Alias;

mod display;
mod tests;

#[derive(Debug, PartialEq)]
pub struct AliasList {
    pub aliases: Vec<Alias>,
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

    fn get_regex() -> Regex {
        const REGEX_STRING: &str = concat!(
            r"^(?:alias )",            // Lookbehind matching "alias " at start of line
            r"(?<shortcut>\S+)",       // Matches text after "alias " under "="
            r#"(?: ?= ?")"#,           // Matches but does not capture "="
            r#"(?<command>.+)(?:")$"#, // Matches all text between quotes and end of line
        );

        #[allow(clippy::unwrap_used)]
        RegexBuilder::new(REGEX_STRING)
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
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect::<Vec<Alias>>();

        AliasList { aliases }
    }

    pub fn iter(&self) -> (impl Iterator<Item = &Alias> + '_) {
        self.aliases.iter()
    }
}
