
#[cfg(test)]
    use super::*;
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
        assert_ne!(Alias::new("Foo", "Bar"), Alias::new("Baz", "Bor"));
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
        assert_eq!(Alias::is_valid_shortcut(potential_shortcut), is_valid);
    }

    #[rstest]
    fn display_alias() {
        assert_eq!(
            Alias::new("Foo", "Bar").to_string(),
            "alias Foo=\"Bar\"".to_string()
        );
    }