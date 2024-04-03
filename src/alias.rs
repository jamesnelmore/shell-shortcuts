#[derive(Debug, PartialEq)]
pub struct Alias {
    shortcut: String,
    command: String,
}

impl Alias {

    // Public Interface

    pub fn new<S>(shortcut: S, command: S) -> Alias
    where
        S: Into<String>,
    {
        Alias {
            shortcut: shortcut.into(),
            command: command.into(),
        }
    }

    pub fn shortcut(&self) -> &String {
        &self.shortcut
    }

    pub fn set_shortcut(&mut self, new_shortcut: String) -> Option<()> {
        match Alias::is_valid_shortcut(new_shortcut.as_str()) {
            true => {
                self.shortcut = new_shortcut;
                Some(())
            }
            false => None,
        }
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn set_command(&mut self, new_command: String) -> Option<()> {
        
        match Alias::is_valid_command(new_command.as_str()) {
            true => {
                self.command = new_command;
                Some(())
            }
            false => None,
        }

    }

    // Helper Methods

    fn is_valid_shortcut(shortcut: &str) -> bool {
        todo!("Need to know if shortcut can have spaces or nonword characters.")
    }

    fn is_valid_command(command: &str) -> bool {
        todo!("Need to look up spec for valid shell command")
    }
}

#[cfg(test)]
mod test_alias {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn simple_equality() {
        let shortcut = "Foo";
        let command = "Bar";

        let alias1 = Alias::new(shortcut, command);
        let alias2 = Alias::new(shortcut, command);
        assert_eq!(alias1, alias2);
    }

    #[rstest]
    fn simple_inequality() {
        assert_ne!(Alias::new("Foo", "Bar"), Alias::new("Baz", "Bor"));
    }

    #[rstest]
    #[case::single_word("hello", true)]
    #[case::two_words("foo bar", false)]
    #[case::numbers("foo3", true)]
    #[case::starting_number("3foo", true)]
    #[case::tab("\tthing", false)]
    #[case::quotes("\"this_is_in_quotes\"", false)]
    #[case::empty_quotes("\"\"", false)]
    fn shortcut_with_spaces(#[case] potential_shortcut: &str, #[case] is_valid: bool) {
        assert_eq!(Alias::is_valid_shortcut(potential_shortcut), is_valid);
    }
}
