mod display;
mod tests;

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
        if Alias::is_valid_shortcut(new_shortcut.as_str()) {
            self.shortcut = new_shortcut;
        }
        None
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn set_command(&mut self, new_command: String) -> Option<()> {
        if Alias::is_valid_command(new_command.as_str()) {
            self.command = new_command;
        }
        None
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
}