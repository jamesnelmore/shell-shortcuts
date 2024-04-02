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

#[cfg(test)]
mod test_alias {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn simple_equality(
        #[values("Foo", "Bar", "foo", "bar", "baz2")] shortcut: &str,

        #[values("Foo", "Bar", "a command", "A. Nother command")] command: &str,
    ) {
        // TODO Parameterize with more cases
        let alias1 = Alias::new(shortcut, command);
        let alias2 = Alias::new(shortcut, command);
        assert_eq!(alias1, alias2);
    }

    #[rstest]
    fn simple_inequality() {
        // TODO parameterize with more cases
        assert_ne!(Alias::new("Foo", "Bar"), Alias::new("Baz", "Bor"));
    }
}
