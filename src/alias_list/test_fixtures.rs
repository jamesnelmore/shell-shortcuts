#![cfg(test)]

use crate::alias::Alias;
use crate::alias_list::AliasList;
use indoc::indoc;
use rstest::fixture;
use tempdir::TempDir;
use std::path::PathBuf;

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

#[fixture]
pub fn temp_directory() -> TempDir {
    #[allow(clippy::unwrap_used)] // REASON: fine if tests fail
    tempdir::TempDir::new("test_directory").unwrap()
}

#[fixture]
pub fn temp_alias_path() -> PathBuf {
    // Path to file containing sample_buf()
    let temp_dir = temp_directory();
    let file_path = temp_dir.into_path().join(".aliases");
    #[allow(clippy::expect_used)]
    std::fs::write(&file_path, sample_buf()).expect("Test cannot run without file contents");

    file_path
}
