mod display;
mod validation;
mod test_fixtures;
use crate::{Error};


use validation::{is_valid_shortcut, is_valid_command};

#[derive(Debug, PartialEq, Eq)]
pub struct Alias {
    shortcut: String,
    command: String,
}

impl Alias { // Public Interface
    pub fn new<S>(shortcut: S, command: S) -> Result<Alias, Error>
    where
        S: Into<String>,
    {
        let shortcut = shortcut.into();
        let command = command.into();

        if !is_valid_shortcut(&shortcut) {
            return Err(Error::InvalidShortcut);
        }

        if !is_valid_command(&command) {
            return Err(Error::InvalidCommand);
        }

        // TODO implement and test for case InvalidShortcutAndCommand

        Ok(Alias { shortcut, command })
    }

    pub fn shortcut(&self) -> &String {
        &self.shortcut
    }

    // TODO test set_shorcut with valid and invalid commands
    // TODO change set_shortcut to return Result
    pub fn set_shortcut(&mut self, new_shortcut: String) -> Option<()> {
        if is_valid_shortcut(new_shortcut.as_str()) {
            self.shortcut = new_shortcut;
        }
        None
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    // TODO test set_command with valid and invalid commands
    // TODO change set_command to return Result
    pub fn set_command(&mut self, new_command: String) -> Option<()> {
        if is_valid_command(new_command.as_str()) {
            self.command = new_command;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::alias::{is_valid_shortcut, Alias};
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
        assert_ne!(
            Alias::new("Foo", "Bar").unwrap(),
            Alias::new("Baz", "Bor").unwrap()
        );
    }
}
