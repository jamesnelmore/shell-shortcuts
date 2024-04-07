use regex::{Regex, RegexBuilder};
use std::path::Path;

use crate::alias::Alias;

mod display;
mod creation;
mod test_fixtures;

#[derive(Debug, PartialEq)]
pub struct AliasList {
    pub aliases: Vec<Alias>,
}

// Public
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
    
    // TODO write remove and replace

    pub fn iter(&self) -> (impl Iterator<Item = &Alias> + '_) {
        self.aliases.iter()
    }
}

#[derive(Debug, Clone)]
pub struct AliasError;
// TODO merge AliasError and AliasParseError
impl std::error::Error for AliasError {}

impl std::fmt::Display for AliasError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is an error") // TODO improve
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Captures;
    use rstest::rstest;
    use test_fixtures::*;
    
    // TODO test add_alias

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
}
