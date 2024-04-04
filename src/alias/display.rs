use std::fmt::{Display, Formatter, Result};
use super::Alias;

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "alias {}=\"{}\"",
            self.shortcut.as_str(),
            self.command.as_str()
        )
    }
}

