#![cfg(test)]
use super::*;
use indoc::indoc;
use regex::Captures;
use rstest::{fixture, rstest};


#[fixture]
fn sample_buf() -> &'static str {
    // TODO parameterize with more cases
    indoc! {r#"
          alias scut="cmd"
          alias thing="Do this"
          alias gs="git status"
        "#}
}

#[fixture]
fn sample_aliases() -> AliasList {
    // TODO parameterize with more cases
    AliasList {
        aliases: vec![
            Alias::new("scut", "cmd").unwrap(),
            Alias::new("thing", "Do this").unwrap(),
            Alias::new("gs", "git status").unwrap(),
        ],
    }
}

#[rstest]
fn iterator_works(sample_aliases: AliasList) {
    let expected_aliases = vec![
        Alias::new("scut", "cmd").unwrap(),
        Alias::new("thing", "Do this").unwrap(),
        Alias::new("gs", "git status").unwrap(),
    ];
    assert_eq!(expected_aliases, sample_aliases.aliases); // Ensure conditions for test are
                                                          // valid. TODO refactor so this is not necessary.

    let mut iter = sample_aliases.iter();

    for alias in &expected_aliases {
        assert_eq!(alias, iter.next().unwrap());
    }
}

#[rstest]
fn display_aliases(sample_aliases: AliasList, sample_buf: &str) {
    assert_eq!(sample_aliases.to_string(), sample_buf);
}
fn match_text(haystack: &str) -> Option<Captures> {
    // TODO parameterize with more cases
    AliasList::get_regex().captures(haystack)
}

// Test Regular Expression

#[rstest]
fn parse_regex_single_example() {
    // TODO parameterize with more cases
    let haystack = r#"alias scut="Do this""#;

    match match_text(haystack) {
        Some(capture) => {
            assert_eq!(&capture["shortcut"], "scut");
            assert_eq!(&capture["command"], "Do this");
        }
        None => panic!("Expected a match"),
    }
}

#[rstest]
fn parse_regex_spaces_around_equals() {
    // TODO parameterize with more cases
    let captures = match_text(r#"alias scut = "Do this"#);
    assert!(Option::is_none(&captures)); // Returns none when there is no match
}

#[rstest]
fn aliases_from_buf_happy_path(sample_buf: &str, sample_aliases: AliasList) {
    // TODO parameterize with more cases
    assert_eq!(AliasList::aliases_from_buf(sample_buf), sample_aliases);
}

#[rstest]
fn add_alias_happy_path() {
    // TODO parameterize by empty and nonempty lists

    let mut aliases = AliasList::new();
    aliases.add_alias(Alias::new("gs", "git status").unwrap());
    assert_eq!(aliases.aliases, vec![Alias::new("gs", "git status").unwrap()]);
}

#[rstest]
fn add_alias_duplicate(mut sample_aliases: AliasList) {
    // TODO parameterize with more cases
    let alias = Alias::new("gs", "git status").unwrap();
    assert!(
        sample_aliases.aliases.contains(&alias),
        "Test conditions not met. Ensure `alias` is in `sample_buf_aliases`."
    );

    assert_eq!(sample_aliases.add_alias(alias), None);
}

