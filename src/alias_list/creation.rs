use std::path::PathBuf;
use regex::{Regex, RegexBuilder};
use crate::alias::Alias;
use crate::alias_list::{AliasError, AliasList};

const REGEX_STRING: &str = concat!(
r"^(?:alias )",            // Lookbehind matching "alias " at start of line
r"(?<shortcut>\S+)",       // Matches text after "alias " under "="
r#"(?: ?= ?")"#,           // Matches but does not capture "="
r#"(?<command>.+)(?:")$"#, // Matches all text between quotes and end of line
);


fn regex() -> Regex {
    RegexBuilder::new(REGEX_STRING)
    .multi_line(true)
    .build()
    .unwrap()}

impl TryFrom<PathBuf> for AliasList {
    type Error = AliasError;
    fn try_from(value: PathBuf) -> Result<Self, AliasError> {
        let buffer = std::fs::read_to_string(value).map_err(|_err| AliasError)?;
        AliasList::try_from(buffer.as_str())
    }
}

impl TryFrom<&str> for AliasList {
    type Error = AliasError;
    fn try_from(value: &str) -> Result<AliasList, Self::Error> {
        // TODO consider returning error
        // TODO write errors

        let aliases = regex()
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
    use regex::Captures;
    use crate::alias_list::AliasList;
    use rstest::rstest;
    use crate::alias::Alias;
    use super::super::test_fixtures::*;

    // TODO write tests

    fn match_text(haystack: &str) -> Option<Captures> {
        // TODO parameterize with more cases
        super::regex().captures(haystack)
    }

// Test Regular Expression

    #[rstest]
    fn parse_regex_single_example() {
        // TODO parameterize with more cases
        let haystack = r#"alias scut="Do this""#;

        match match_text(haystack) {
            Some(capture) => {
                assert_eq!(&capture["shortcut"], "scut");
                assert_eq!(&capture["command"], "Do this");
            }
            None => panic!("Expected a match"),
        }
    }

    #[rstest]
    fn parse_regex_spaces_around_equals() {
        // TODO parameterize with more cases
        let captures = match_text(r#"alias scut = "Do this"#);
        assert!(Option::is_none(&captures)); // Returns none when there is no match
    }

    #[rstest]
    fn aliases_from_buf_happy_path(
        sample_buf: &str,
        sample_aliases: AliasList,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        // TODO parameterize with more cases
        assert_eq!(AliasList::try_from(sample_buf)?, sample_aliases);
        Ok(())
    }

    #[rstest]
    fn add_alias_happy_path() {
        // TODO parameterize by empty and nonempty lists

        let mut aliases = AliasList::new();
        aliases.add_alias(Alias::new("gs", "git status").unwrap());
        assert_eq!(
            aliases.aliases,
            vec![Alias::new("gs", "git status").unwrap()]
        );
    }

    #[rstest]
    fn add_alias_duplicate(mut sample_aliases: AliasList) {
        // TODO parameterize with more cases
        let alias = Alias::new("gs", "git status").unwrap();
        assert!(
            sample_aliases.aliases.contains(&alias),
            "Test conditions not met. Ensure `alias` is in `sample_buf_aliases`."
        );

        assert_eq!(sample_aliases.add_alias(alias), None);
    }
}