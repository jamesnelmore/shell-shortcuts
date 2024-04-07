pub fn is_valid_shortcut(shortcut: &str) -> bool {
    // TODO improve readability and extensibility
    !(shortcut.contains(' ')
        || shortcut.contains('\t')
        || shortcut.contains('\n')
        || shortcut.contains('"'))
        && shortcut.is_ascii()
}

pub fn is_valid_command(command: &str) -> bool {
    // TODO improve
    command.is_ascii()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::single_word("hello", true)]
    #[case::two_words("foo bar", false)]
    #[case::numbers("foo3", true)]
    #[case::starting_number("3foo", true)]
    #[case::tab("\tthing", false)]
    #[case::quotes("\"this_is_in_quotes\"", false)]
    #[case::empty_quotes("\"\"", false)]
    fn shortcut_with_spaces(#[case] potential_shortcut: &str, #[case] is_valid: bool) {
        // TODO rename test
        assert_eq!(is_valid_shortcut(potential_shortcut), is_valid);
    }
    
    // TODO test is_valid_command similarly
}