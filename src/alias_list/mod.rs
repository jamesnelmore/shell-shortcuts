use crate::{Alias, Error};
use std::path::PathBuf;

mod creation;
mod display;
mod test_fixtures;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

    pub fn remove_alias_by_shortcut(&mut self, shortcut: &str) -> Result<(), Error> {
        let old_index = self
            .iter()
            .position(|alias| alias.shortcut() == shortcut)
            .ok_or(Error::InvalidShortcut)?;
        self.aliases.remove(old_index);
        Ok(())
    }

    pub fn replace_shortcut(&mut self, old: &str, new: String) -> Result<(), Error> {
        let alias: &mut Alias = self
            .iter_mut()
            .find(|alias| alias.shortcut() == old)
            .ok_or(Error::InvalidShortcut)?;
        alias.set_shortcut(new)
    }

    pub fn save_to_file(&self, path: PathBuf) -> Result<(), Error> {
        std::fs::write(path, self.to_string()).map_err(Error::IOError)
    }

    // TODO write remove and replace

    pub fn iter(&self) -> (impl Iterator<Item = &Alias> + '_) {
        self.aliases.iter()
    }

    pub fn iter_mut(&mut self) -> (impl Iterator<Item = &mut Alias> + '_) {
        self.aliases.iter_mut()
    }
}

impl FromIterator<Alias> for AliasList {
    fn from_iter<T>(iter: T) -> Self  where T: IntoIterator<Item = Alias>{
        let iter = iter.into_iter();
        
        AliasList {
            aliases: iter.collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;
    use test_fixtures::*;

    // TODO test add_alias

    #[rstest]
    fn iterator_works(sample_aliases: AliasList) -> Result<(), Error> {
        let expected_aliases = vec![
            Alias::new("scut", "cmd")?,
            Alias::new("thing", "Do this")?,
            Alias::new("gs", "git status")?,
        ];
        assert_eq!(expected_aliases, sample_aliases.aliases); // Ensure conditions for test are
                                                              // valid. TODO refactor so this is not necessary.

        let mut iter = sample_aliases.iter();

        for alias in &expected_aliases {
            #[allow(clippy::unwrap_used)] // Should not fail because length of expected_aliases
            // should equal length of iterator
            let next = iter.next().unwrap();
            assert_eq!(alias, next);
        }
        assert_eq!(iter.next(), None);

        Ok(())
    }
}
