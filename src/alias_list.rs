use std::path::PathBuf;
// use std::io::{Read, self};
use crate::alias::Alias;
use regex::Regex;
use std::fs::{self};
use std::vec::Vec;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AliasList {
    aliases: Vec<Alias>,
    path: PathBuf,
}

#[allow(dead_code)]
impl AliasList {
    fn new(aliases: Vec<Alias>, path: PathBuf) -> Self {
        AliasList { aliases, path }
    }

    pub fn new_from_file(path: impl Into<PathBuf>) -> AliasList {
        let file_contents = fs::read(path.into());
        todo!();
    }
    const fn aliases_from_buffer(buf: impl Into<String>) -> Option<Vec<Alias>> {
        const REGEX: Regex =
            Regex::new(r#"(?:alias )(?<shortcut>\S+)(?: ?= ?")(?<command>\S+)(?:")"#).unwrap(); // TODO
                                                                                                // document regex
        Some(
            buf.into()
                .lines()
                .map(|line| REGEX.captures(line).unwrap()) // TODO change to result
                .map(|capture| {
                    Alias::new(
                        capture["shortcut"].to_string(),
                        capture["command"].to_string(),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn get_aliases(&self) -> Vec<Alias> {
        // Return collection of Aliases in file
        self.aliases.clone()
    }

    fn add_alias(&mut self, new: Alias) -> Option<()> {
        // Error if duplicate shortcut
        match &self.aliases.contains(&new) {
            false => {
                self.aliases.push(new);
                return Some(());
            }
            true => return None,
        }
    }

    #[allow(unused_variables)]
    fn remove_shortcut(&mut self, shortcut: impl Into<String>) -> Option<()> {
        todo!();
    }

    #[allow(unused_variables)]
    fn replace_alias(&mut self, new: Alias) -> Option<()> {
        todo!();
    }

    fn contains(&self, candidate: &Alias) -> bool {
        self.aliases.contains(&candidate)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use rstest::{fixture, rstest};
    use std::fs::File;
    use std::io::Write;
    use tempdir::{self, TempDir};

    #[fixture]
    const fn an_alias_str() -> &'static str {
        indoc! {r#"
    alias alias_name = "command_to_run"
    alias test_command="anotherthing""#}
    }
    #[fixture]
    fn empty_alias_list() -> AliasList {
        AliasList::new(vec![])
    }
    #[fixture]
    fn an_alias_list() -> AliasList {
        AliasList::new(vec![
            Alias::new("alias_name", "command_to_run"),
            Alias::new("test_command", "anotherthing"),
        ])
    }

    #[fixture]
    fn an_alias_in_an_alias_list() -> Alias {
        Alias::new("alias_name", "command_to_run")
    }

    #[rstest]
    fn test_aliaslist_from_buffer(an_alias_list: AliasList, an_alias_str: &str) {
        let alias_list = AliasList::new_from_buffer(an_alias_str);
        println!("{:?}", alias_list);
        assert_eq!(an_alias_list, alias_list);
    }

    #[rstest]
    fn test_aliaslist_from_file(an_alias_list: AliasList, an_alias_str: &str) {
        let tmp_dir = TempDir::new("test_dir").unwrap();
        let tmp_path = tmp_dir.path().join("tmp_file");
        let mut tmp_file = File::create(&tmp_path).unwrap();
        tmp_file.write(an_alias_str.as_bytes()).unwrap();

        let test_list = AliasList::new_from_file(&tmp_path);
        assert_eq!(test_list, an_alias_list);
    }

    #[rstest]
    fn test_add_alias_empty(mut empty_alias_list: AliasList) {
        let an_alias = Alias::new("shortcut", "command");
        empty_alias_list.add_alias(an_alias);

        let expected_alias_list = AliasList {
            aliases: vec![Alias::new("shortcut", "command")],
        };

        assert_eq!(empty_alias_list, expected_alias_list);
    }

    #[rstest]
    fn test_alias_list_contains_does_contain(an_alias_list: AliasList) {
        assert!(an_alias_list.contains(&Alias::new("alias_name", "command")))
    }
    #[rstest]
    fn test_alias_list_contains_does_not_contain(an_alias_list: AliasList) {
        assert_eq!(an_alias_list.contains(&Alias::new("a", "thing")), false)
    }

    #[rstest]
    fn test_add_alias_no_duplicate(mut an_alias_list: AliasList) {
        let new_alias = Alias::new("shortcut", "command");
        assert_eq!(
            an_alias_list.add_alias(new_alias),
            Some(()),
            "add_alias() not return Some(())"
        );

        let expected_alias_list = AliasList::new(vec![
            Alias::new("alias_name", "command_to_run"),
            Alias::new("test_command", "anotherthing"),
            Alias::new("shortcut", "command"),
        ]);

        assert_eq!(
            an_alias_list, expected_alias_list,
            "add_alias() did not add alias"
        )
    }

    #[rstest]
    fn test_add_alias_duplicate(mut an_alias_list: AliasList) {
        let new_alias = Alias::new("test_command", "anotherthing");
        assert!(an_alias_list.add_alias(new_alias).is_none());
    }

    #[rstest]
    fn test_replace_alias_does_exist(
        mut an_alias_list: AliasList,
        an_alias_in_an_alias_list: Alias,
    ) {
        assert!(an_alias_list
            .replace_alias(an_alias_in_an_alias_list)
            .is_some());
        todo!("Actually change alias and check for containment");
    }

    #[rstest]
    fn test_replace_alias_does_not_exist(mut an_alias_list: AliasList) {
        let replacement = Alias::new("notin", "thing");
        assert!(an_alias_list.replace_alias(replacement).is_none());
    }

    #[rstest]
    fn test_replace_alias_empty_list(mut empty_alias_list: AliasList) {
        let replacement = Alias::new("notin", "thing");
        assert!(empty_alias_list.replace_alias(replacement).is_none());
    }

    #[rstest]
    #[ignore]
    fn test_remove_alias_does_exist(mut an_alias_list: AliasList) {
        an_alias_list.remove_shortcut("test_command").unwrap();
        todo!("Check for containment")
    }

    #[rstest]
    #[ignore]
    fn test_remove_alias_does_not_exist() {
        todo!();
    }

    #[rstest]
    #[ignore]
    fn test_get_aliases() {
        todo!();
    }

    #[rstest]
    #[ignore]
    fn test_get_aliases_empty() {
        todo!();
    }
}
