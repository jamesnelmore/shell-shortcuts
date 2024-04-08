#![cfg(test)]

use crate::alias::Alias;
use crate::alias_list::AliasList;
use indoc::indoc;
use rstest::fixture;

// All fixtures must be public

#[fixture]
pub fn sample_buf() -> &'static str {
    // TODO parameterize with more cases
    indoc! {r#"
          alias scut="cmd"
          alias thing="Do this"
          alias gs="git status"
        "#}
}

#[fixture]
pub fn sample_aliases() -> AliasList {
    // TODO parameterize with more cases
    #![allow(clippy::unwrap_used)] // From constants so cannot fail.
    AliasList {
        aliases: vec![
            Alias::new("scut", "cmd").unwrap(),
            Alias::new("thing", "Do this").unwrap(),
            Alias::new("gs", "git status").unwrap(),
        ],
    }
}

#[fixture]
pub fn sample_alias() -> Alias {
    #[allow(clippy::unwrap_used)]
    Alias::new("gs", "git status").unwrap()
}
