use regex::{Regex, RegexBuilder};

use crate::alias::Alias;

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
        if self.aliases.iter().any(|a| a.shortcut() == alias.shortcut()) {
            return None;
        }
        self.aliases.push(alias);

        Some(())
    }

    #[allow(unused)]
    pub fn to_buffer(&self) -> String {
        self.aliases
            .iter()
            .map(|alias| format!("alias {}=\"{}\"", alias.shortcut(), alias.command()))
            .collect::<Vec<_>>()
            .join("\n")
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
}

// todo!("Write iterator for AliasList");
// Impl std::Iter for AliasList {
//     type Alias;
//     fn next(&mut self) -> Option<Self::Item
// }

#[cfg(test)]
mod test_alias_list {
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
                Alias::new("scut", "cmd"),
                Alias::new("thing", "Do this"),
                Alias::new("gs", "git status"),
            ],
        }
    }

    fn match_text(haystack: &str) -> Option<Captures> {
        // TODO parameterize with more cases
        AliasList::get_regex().captures(haystack)
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
    fn aliases_from_buf_happy_path(sample_buf: &str, sample_aliases: AliasList) {
        // TODO parameterize with more cases
        assert_eq!(AliasList::aliases_from_buf(sample_buf), sample_aliases);
    }

    #[rstest]
    fn add_alias_happy_path() {
        // TODO parameterize by empty and nonempty lists

        let mut aliases = AliasList::new();
        aliases.add_alias(Alias::new("gs", "git status")).unwrap();
        assert_eq!(aliases.aliases, vec![Alias::new("gs", "git status")]);
    }

    #[rstest]
    fn add_alias_duplicate(mut sample_aliases: AliasList) {
        // TODO parameterize with more cases
        let alias = Alias::new("gs", "git status");
        assert!(
            sample_aliases.aliases.contains(&alias),
            "Test conditions not met. Ensure `alias` is in `sample_buf_aliases`."
        );

        assert_eq!(sample_aliases.add_alias(alias), None);
    }
}

