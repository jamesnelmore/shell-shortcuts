use std::fmt::{Display, Formatter, Result};
use super::AliasList;

impl Display for AliasList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let display = self
            .aliases
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", display + "\n")
    }
}
