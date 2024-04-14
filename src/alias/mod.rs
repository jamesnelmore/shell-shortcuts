mod display;
mod validation;
use crate::Error;

use serde::{Deserialize, Serialize};
use validation::{is_valid_command, is_valid_shortcut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Alias {
    shortcut: String,
    command: String,
}

impl Alias {
    // Public Interface
    pub fn new<S>(shortcut: S, command: S) -> Result<Alias, Error>
    where
        S: Into<String>,
    {
        let shortcut: String = shortcut.into();
        let command: String = command.into();

        if !is_valid_shortcut(&shortcut) {
            return Err(Error::InvalidShortcut(shortcut));
        }

        if !is_valid_command(&command) {
            return Err(Error::InvalidCommand(command));
        }

        Ok(Alias { shortcut, command })
    }

    pub fn shortcut(&self) -> &String {
        &self.shortcut
    }

    // TODO test set_shorcut with valid and invalid commands
    // TODO change set_shortcut to return Result
    pub fn set_shortcut(&mut self, new_shortcut: String) -> Result<(), Error> {
        if is_valid_shortcut(new_shortcut.as_str()) {
            self.shortcut = new_shortcut;
            Ok(())
        } else {
            Err(Error::InvalidShortcut(new_shortcut))
        }
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    // TODO test set_command with valid and invalid commands
    // TODO change set_command to return Result
    #[allow(dead_code)]
    pub fn set_command(&mut self, new_command: String) -> Result<(), Error> {
        if is_valid_command(new_command.as_str()) {
            self.command = new_command;
            Ok(())
        } else {
            Err(Error::InvalidCommand(new_command))
        }
    }
}

impl Clone for Alias {
    fn clone(&self) -> Self {
        Alias {
            shortcut: self.shortcut().clone(),
            command: self.command().clone(),
        }
        // TODO test
    }
}

#[cfg(test)]
mod test {
    use crate::alias::Alias;
    use crate::Error;
    use rstest::rstest;

    #[rstest]
    fn simple_equality() -> Result<(), Error> {
        let shortcut = "Foo";
        let command = "Bar";

        let alias1 = Alias::new(shortcut, command)?;
        let alias2 = Alias::new(shortcut, command)?;

        assert_eq!(alias1, alias2);

        Ok(())
    }

    #[rstest]
    fn simple_inequality() -> Result<(), Error> {
        assert_ne!(Alias::new("Foo", "Bar")?, Alias::new("Baz", "Bor")?);

        Ok(())
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Alias>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Alias>();
    }
}
