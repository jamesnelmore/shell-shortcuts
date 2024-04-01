use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub shortcut: String,
    pub command: String,
}

impl Alias {
    pub fn new<S>(shortcut: S, command: S) -> Alias
    where
        S: Into<String>,
    {
        Alias {
            shortcut: shortcut.into(),
            command: command.into(),
        }
    }
}

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
        if self.aliases.iter().any(|a| a.shortcut == alias.shortcut) {
            return None;
        }
        self.aliases.push(alias);

        Some(())
    }
    #[allow(unused)]
    pub fn to_buffer(&self) -> String {
        self.aliases
            .iter()
            .map(|alias| format!("alias {}=\"{}\"", alias.shortcut, alias.command))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[allow(unused)]
    const REGEX_STRING: &'static str = concat!(
        r"^(:alias )",             // Lookbehind matching "alias " at start of line
        r"(?<shortcut>.+)",        // Matches text after "alias " under "="
        r#"(?: ?= ?")"#,           // Matches but does not capture "="
        r#"(?<command>.+)(?:")$"#, // Matches all text between quotes and end of line
    );

    #[allow(unused)]
    fn get_regex() -> Regex {
        #[allow(clippy::unwrap_used)]
        RegexBuilder::new(AliasList::REGEX_STRING)
            .multi_line(true)
            .build()
            .unwrap()
    }

    #[allow(unused)]
    fn aliases_from_buf(buf: &str) -> AliasList {
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

#[cfg(test)]
mod test_alias_list {
    use super::*;
    use indoc::indoc;
    use rstest::{fixture, rstest};

    #[fixture]
    fn sample_buf() -> &'static str {
        indoc! {r#"
          alias cmd="Shortcut"
          alias thing="Do this"
          alias gs="git status"
        "#}
    }

    #[fixture]
    fn sample_buf_aliases() -> AliasList {
        AliasList {
            aliases: vec![
                Alias::new("cmd", "Shortcut"),
                Alias::new("thing", "Do this"),
                Alias::new("gs", "git status"),
            ],
        }
    }

    #[rstest]
    fn aliases_from_buf_happy_path(sample_buf: &str, sample_buf_aliases: AliasList) {
        assert_eq!(AliasList::aliases_from_buf(sample_buf), sample_buf_aliases);
    }
}

#[cfg(test)]
mod test_alias {
    use super::*;

    #[test]
    fn test_alias_equality() {
        let alias1 = Alias::new("Foo", "Bar");
        let alias2 = Alias::new("Foo", "Bar");
        assert_eq!(alias1, alias2);
    }

    #[test]
    fn test_alias_inequality() {
        assert_ne!(Alias::new("Foo", "Bar"), Alias::new("Baz", "Bor"));
    }
}
