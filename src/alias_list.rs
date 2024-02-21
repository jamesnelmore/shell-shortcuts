use std::path::Path;
use std::io::{Read, self};
use std::fs::{self, File};
use std::vec::Vec;
use regex::Regex;
use crate::alias::Alias;


#[derive(PartialEq, Eq, Hash, Debug)]
struct AliasList {
    aliases: Vec<Alias>
}

impl AliasList {
    fn new(vector: Vec<Alias>) -> AliasList {
        AliasList{aliases: vector}
    }

    fn new_from_buffer(buf: impl Into<String>) -> AliasList {
        let regex_str = r#"(?:alias )(?<shortcut>\S+)(?: ?= ?")(?<command>\S+)(?:")"#;
        let alias_regex: Regex = Regex::new(regex_str).unwrap();

        // iterate over lines and extract capture groups from each
        let mut alias_vec = Vec::new();
        for line in buf.into().lines() {
            let capture = alias_regex.captures(line).expect("Failed. Invalid alias expressions");
            alias_vec.push(Alias::new(capture["shortcut"].to_string(), capture["command"].to_string()));
        }
        AliasList::new(alias_vec)
    }

    fn new_from_file(path: &Path) -> AliasList {
        //get file as string 
        // pass string to new_from_buffer

        let file = fs::read(path)
            .expect("Could not read file");
        let file_contents = String::from_utf8(file).expect("Invalid UTF-8");

        AliasList::new_from_buffer(file_contents)
    }

    fn get_aliases(&self) -> Vec<Alias> {
        // Return collection of Aliases in file
        self.aliases.clone()
    }

    fn add_alias(&mut self, new: Alias) -> Option<()> { // Error if duplicate shortcut
        match &self.aliases.contains(&new) {
            false => {
                self.aliases.push(new);
                return Some(())
            }
            true => { return None }
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
    use rstest::{rstest, fixture};
    use std::fs::File;
    use std::io::Write;
    use indoc::indoc;
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
        Alias::new("test_command", "anotherthing")
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

        let expected_alias_list = AliasList{
            aliases: vec![Alias::new("shortcut", "command")]
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
        assert_eq!(an_alias_list.add_alias(new_alias), Some(()), 
            "add_alias() not return Some(())");

        let expected_alias_list = AliasList::new(vec![
        Alias::new("alias_name", "command_to_run"),
        Alias::new("test_command", "anotherthing"),
        Alias::new("shortcut", "command")
        ]);

        assert_eq!(an_alias_list, expected_alias_list, "add_alias() did not add alias")
    }

    #[rstest]
    fn test_add_alias_duplicate(mut an_alias_list: AliasList) {
        let new_alias = Alias::new("test_command", "anotherthing");
        assert!(an_alias_list.add_alias(new_alias).is_none());
    }

    #[rstest]
    fn test_replace_alias_does_exist(mut an_alias_list: AliasList,
        an_alias_in_an_alias_list: Alias) {
        assert!(an_alias_list.replace_alias(an_alias_in_an_alias_list).is_some());
        todo!("Actually change alias and check for containment");
    }

    #[rstest]
    fn test_replace_alias_does_not_exist (mut an_alias_list: AliasList) {
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
    fn test_remove_alias_does_not_exist() {
        todo!();
    }

    #[rstest]
    fn test_get_aliases() {
        todo!();
    }

    #[rstest]
    fn test_get_aliases_empty() {
        todo!();
    }
}
