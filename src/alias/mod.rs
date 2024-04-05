mod display;
mod tests;

#[derive(Debug, PartialEq)]
pub struct Alias {
    shortcut: String,
    command: String,
}

#[allow(clippy::enum_variant_names, clippy::module_name_repetitions)] // TODO See if implementing error fixes this, else rename to
// be valid
#[derive(Debug, PartialEq)]
pub enum AliasParseError {
    InvalidShortcut,
    InvalidCommand,
    InvalidShortcutAndCommand,
}

impl Alias {
    // Public Interface

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

    pub fn set_shortcut(&mut self, new_shortcut: String) -> Option<()> {
        if is_valid_shortcut(new_shortcut.as_str()) {
            self.shortcut = new_shortcut;
        }
        None
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn set_command(&mut self, new_command: String) -> Option<()> {
        if is_valid_command(new_command.as_str()) {
            self.command = new_command;
        }
        None
    }
}

// Helper Methods
// TODO move helper methods and associated tests to own module
// TODO Make test module with sharable fixtures

fn is_valid_shortcut(shortcut: &str) -> bool {
    // TODO improve readability and extensibility
    !(shortcut.contains(' ')
        || shortcut.contains('\t')
        || shortcut.contains('\n')
        || shortcut.contains('"'))
        && shortcut.is_ascii()
}

fn is_valid_command(command: &str) -> bool {
    // TODO improve
    command.is_ascii()
}
