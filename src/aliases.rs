use regex::Regex;

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

#[derive(Debug)]
pub struct AliasList {
    pub aliases: Vec<Alias>,
}

impl AliasList {
    pub fn new() -> AliasList {
        AliasList { aliases: vec![] }
    }

    pub fn add_alias(&mut self, alias: Alias) -> Option<()> {
        if self.aliases.iter().any(|a| a.shortcut == alias.shortcut) {
            return None;
        }
        self.aliases.push(alias);

        Some(())
    }

    pub fn to_buffer(&self) -> String {
        self.aliases
            .iter()
            .map(|alias| format!("alias {}=\"{}\"", alias.shortcut, alias.command))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn aliases_from_buf(buf: &str) -> AliasList {
    // TODO Document REGEX
    const REGEX_STRING: &str = r#"^(?:alias )(?<shortcut>\S+)(?: ?= ?")(?<command>\S+)(?:")$"#;

    #[allow(clippy::unwrap_used)]
    let regex: Regex = Regex::new(REGEX_STRING).unwrap();

    let aliases = regex
        .captures_iter(buf)
        .map(|capture| {
            Alias::new(
                capture["shortcut"].to_string(),
                capture["command"].to_string(),
            )
        })
        .collect::<Vec<Alias>>();

    AliasList { aliases }
}

#[cfg(test)]
mod test_alias_list {
    use super::*;
    use indoc;
    use rstest::{fixture};

    #[fixture]
    fn sample_buf() -> &'static str {
        indoc! {r#"
          alias cmd="Shortcut"
          alias thing="Do this"
          alias gs="git status"
        "#}
    }

    #[test]
    fn aliases_from_buf_happy_path(sample_buf: &str) {
        let aliases = aliases_from_buf(sample_buf);
        let test_buffer = aliases.to_buffer();

        assert_eq!(sample_buf, test_buffer);
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
