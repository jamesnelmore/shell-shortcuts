mod display;
mod validation;
mod test_fixtures;

use std::fmt::Formatter;
use validation::{is_valid_shortcut, is_valid_command};

#[derive(Debug, PartialEq)]
pub struct Alias {
    shortcut: String,
    command: String,
}

impl Alias { // Public Interface
    pub fn new<S>(shortcut: S, command: S) -> Result<Alias, AliasParseError>
    where
        S: Into<String>,
    {
        let shortcut = shortcut.into();
        let command = command.into();

        if !is_valid_shortcut(&shortcut) {
            return Err(AliasParseError::InvalidShortcut);
        }

        if !is_valid_command(&command) {
            return Err(AliasParseError::InvalidCommand);
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

#[allow(clippy::enum_variant_names, clippy::module_name_repetitions)]
// TODO See if implementing error fixes this, else rename to
// be valid
#[derive(Debug, PartialEq)]
pub enum AliasParseError {
    InvalidShortcut,
    InvalidCommand,
    InvalidShortcutAndCommand,
} // TODO test for failing parses

impl std::error::Error for AliasParseError {}

impl std::fmt::Display for AliasParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(()) // TODO actually implement
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
