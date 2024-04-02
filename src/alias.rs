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
