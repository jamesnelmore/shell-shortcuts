#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Alias {
    shortcut: String,
    command: String,
}

impl Alias {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(shortcut: S, command: S) -> Alias {
        Alias {
            shortcut: shortcut.into(),
            command: command.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alias_equality() {
        let alias1 = Alias::new("Foo", "Bar");
        let alias2 = Alias::new("Foo", "Bar");
        assert_eq!(alias1, alias2);
    }

    #[test]
    fn test_alias_inequality() {
        assert_ne!(Alias::new("Foo", "Bar"), Alias::new("Baz", "Bor"))
    }
}
