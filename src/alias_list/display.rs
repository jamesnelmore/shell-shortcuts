use super::AliasList;
use std::fmt::{Display, Formatter, Result};

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

#[cfg(test)]
mod test {
    use super::super::test_fixtures::*;
    use crate::alias_list::AliasList;
    use rstest::rstest;

    #[rstest]
    fn display_aliases(sample_aliases: AliasList, sample_buf: &str) {
        assert_eq!(sample_aliases.to_string(), sample_buf);
    }

    // TODO test if empty alias is valid (shouldn't be)
}
