mod display;

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

    #[rstest]
    #[case::single_word("hello", true)]
    #[case::two_words("foo bar", false)]
    #[case::numbers("foo3", true)]
    #[case::starting_number("3foo", true)]
    #[case::tab("\tthing", false)]
    #[case::quotes("\"this_is_in_quotes\"", false)]
    #[case::empty_quotes("\"\"", false)]
    fn shortcut_with_spaces(#[case] potential_shortcut: &str, #[case] is_valid: bool) {
        assert_eq!(is_valid_shortcut(potential_shortcut), is_valid);
    }
}
