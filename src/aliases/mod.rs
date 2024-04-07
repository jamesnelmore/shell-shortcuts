use regex::{Regex, RegexBuilder};
use std::path::Path;

use crate::alias::Alias;

mod display;
mod regex_tests;

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

    pub fn iter(&self) -> (impl Iterator<Item = &Alias> + '_) {
        self.aliases.iter()
    }

    pub fn new_from_path(value: &Path) -> Result<Self, AliasError> {
        let buffer = std::fs::read_to_string(value).map_err(|_err| AliasError)?;
        AliasList::try_from(buffer.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct AliasError;

impl std::error::Error for AliasError {}

impl std::fmt::Display for AliasError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is an error") // TODO improve
    }
}

impl TryFrom<&str> for AliasList {
    type Error = AliasError;
    fn try_from(value: &str) -> Result<AliasList, Self::Error> {
        // TODO consider returning error
        // TODO write errors

        let aliases = Self::get_regex()
            .captures_iter(value)
            .map(|capture| {
                let shortcut = &capture["shortcut"].to_string();
                let command = &capture["command"].to_string();
                println!("Captured: {shortcut} | {command}");
                Alias::new(shortcut, command)
            })
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect::<Vec<Alias>>();

        if aliases.len() == 0 {
            return Err(AliasError);
        }
        Ok(AliasList { aliases })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use regex::Captures;
    use rstest::{fixture, rstest};

    #[fixture]
    fn sample_buf() -> &'static str {
        // TODO parameterize with more cases
        indoc! {r#"
          alias scut="cmd"
          alias thing="Do this"
          alias gs="git status"
        "#}
    }

    #[fixture]
    fn sample_aliases() -> AliasList {
        // TODO parameterize with more cases
        AliasList {
            aliases: vec![
                Alias::new("scut", "cmd").unwrap(),
                Alias::new("thing", "Do this").unwrap(),
                Alias::new("gs", "git status").unwrap(),
            ],
        }
    }

    #[rstest]
    fn iterator_works(sample_aliases: AliasList) {
        let expected_aliases = vec![
            Alias::new("scut", "cmd").unwrap(),
            Alias::new("thing", "Do this").unwrap(),
            Alias::new("gs", "git status").unwrap(),
        ];
        assert_eq!(expected_aliases, sample_aliases.aliases); // Ensure conditions for test are
                                                              // valid. TODO refactor so this is not necessary.

        let mut iter = sample_aliases.iter();

        for alias in &expected_aliases {
            assert_eq!(alias, iter.next().unwrap());
        }
    }

    #[rstest]
    fn display_aliases(sample_aliases: AliasList, sample_buf: &str) {
        assert_eq!(sample_aliases.to_string(), sample_buf);
    }
    fn match_text(haystack: &str) -> Option<Captures> {
        // TODO parameterize with more cases
        AliasList::get_regex().captures(haystack)
    }
}
