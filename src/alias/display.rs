use super::Alias;
use std::fmt::{Display, Formatter, Result};

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

#[cfg(test)]
mod test {
    use super::Alias;
    use rstest::rstest;

    #[rstest]
    fn display_alias() {
        assert_eq!(
            Alias::new("Foo", "Bar").unwrap().to_string(),
            "alias Foo=\"Bar\"".to_string()
        );
    }

    #[ignore]
    #[rstest]
    fn display_special_characters() {
        todo!()
    }
}
