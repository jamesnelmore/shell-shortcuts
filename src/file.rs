use std::path::Path;
use std::io::Read;
use std::fs::{self, File};
use std::vec::Vec;
use regex::Regex;
// TODO Write struct that contains all aliases and their associated commands

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
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Alias {
    shortcut: String,
    command: String
}

impl Alias {
    fn new<S: Into<String>>(shortcut: S, command: S) -> Alias {
       Alias {
            shortcut: shortcut.into(),
            command: command.into()
        } 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use indoc::indoc;
    use tempdir::{self, TempDir};

        // alias alias_name="command_to_run"
    static ALIAS_STR: &str = indoc! {r#"
    alias alias_name = "command_to_run"
    alias test_command="anotherthing""#};

    fn get_expected_alias_list() -> AliasList {
        AliasList::new(vec![
        Alias::new("alias_name", "command_to_run"),
        Alias::new("test_command", "anotherthing")
        ])
    }
    #[test]
    fn test_alias_equality() {
        let alias1 = Alias::new("Foo", "Bar");
        let alias2 = Alias::new("Foo", "Bar");
        assert_eq!(alias1, alias2);
    }

    #[test]
    fn test_alias_inequality() {
        assert_ne!(
        Alias::new("Foo", "Bar"),
        Alias::new("Baz", "Bor")
    )
    }

    #[test]
    fn test_aliaslist_from_buffer() {
        // Assert that AliasList::from_file correctly interprets a file of shell aliases
        
        // Steps
        // Create a mock stream
        // Push file to mockstream
        // Test

        // let mut stream = MockStream::new();

        
        //
        // stream.push_bytes_to_read(alias_str);
        // let mut buffer = String::new();
        // let _ = stream.read_to_string(&mut buffer).unwrap(); // TODO handle unwrap
        //
        

        println!("{:?}", get_expected_alias_list());
        let alias_list = AliasList::new_from_buffer(ALIAS_STR);
        println!("{:?}", alias_list);
        assert_eq!(get_expected_alias_list(), alias_list);
    }

    #[test]
    fn test_aliaslist_from_file() {
        let tmp_dir = TempDir::new("test_dir").unwrap();
        let tmp_path = tmp_dir.path().join("tmp_file");
        let mut tmp_file = File::create(&tmp_path).unwrap();
        tmp_file.write(ALIAS_STR.as_bytes()).unwrap();

        let test_list = AliasList::new_from_file(&tmp_path);
        assert_eq!(test_list, get_expected_alias_list());
    }
}













