enum RawStyle {
    Plain,
    Strong,
    AsteriskItalic,
    UnderscoreItalic,
    Code,
}

fn check_head_symbol(input: &str) -> RawStyle {
    RawStyle::Plain // placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    // Strong
    #[test]
    fn strong_for_double_asterisk_text() {
        assert!(matches!(check_head_symbol("**text"), RawStyle::Strong));
    }
    #[test]
    fn strong_for_double_asterisk_space() {
        assert!(matches!(check_head_symbol("** "), RawStyle::Strong));
    }
    #[test]
    fn strong_for_double_asterisk_min() {
        assert!(matches!(check_head_symbol("**a"), RawStyle::Strong));
    }

    // Asterisk italic
    #[test]
    fn asterisk_italic_for_single_asterisk_text() {
        assert!(matches!(
            check_head_symbol("*text"),
            RawStyle::AsteriskItalic
        ));
    }
    #[test]
    fn asterisk_italic_for_single_asterisk_space() {
        assert!(matches!(check_head_symbol("* "), RawStyle::AsteriskItalic));
    }
    #[test]
    fn asterisk_italic_for_single_asterisk_min() {
        assert!(matches!(check_head_symbol("*a"), RawStyle::AsteriskItalic));
    }

    // Underscore italic
    #[test]
    fn underscore_italic_for_underscore_text() {
        assert!(matches!(
            check_head_symbol("_text"),
            RawStyle::UnderscoreItalic
        ));
    }
    #[test]
    fn underscore_italic_for_underscore_space() {
        assert!(matches!(
            check_head_symbol("_ "),
            RawStyle::UnderscoreItalic
        ));
    }
    #[test]
    fn underscore_italic_for_underscore_min() {
        assert!(matches!(
            check_head_symbol("_a"),
            RawStyle::UnderscoreItalic
        ));
    }

    // Code
    #[test]
    fn code_for_backtick_text() {
        assert!(matches!(check_head_symbol("`text"), RawStyle::Code));
    }
    #[test]
    fn code_for_backtick_space() {
        assert!(matches!(check_head_symbol("` "), RawStyle::Code));
    }
    #[test]
    fn code_for_backtick_min() {
        assert!(matches!(check_head_symbol("`a"), RawStyle::Code));
    }

    // Plain
    #[test]
    fn plain_for_regular_text() {
        assert!(matches!(check_head_symbol("hello"), RawStyle::Plain));
    }
    #[test]
    fn plain_for_number() {
        assert!(matches!(check_head_symbol("123"), RawStyle::Plain));
    }
    #[test]
    fn plain_for_leading_space() {
        assert!(matches!(check_head_symbol(" text"), RawStyle::Plain));
    }
    #[test]
    fn plain_for_empty_string() {
        assert!(matches!(check_head_symbol(""), RawStyle::Plain));
    }

    // Priority
    #[test]
    fn double_asterisk_is_strong_not_italic() {
        assert!(matches!(check_head_symbol("**bold**"), RawStyle::Strong));
    }
    #[test]
    fn triple_asterisk_is_strong() {
        assert!(matches!(check_head_symbol("***x"), RawStyle::Strong));
    }
    #[test]
    fn quad_asterisk_is_strong() {
        assert!(matches!(check_head_symbol("****x"), RawStyle::Strong));
    }

    // Edge cases
    #[test]
    fn strong_followed_by_underscore() {
        assert!(matches!(check_head_symbol("**_"), RawStyle::Strong));
    }
    #[test]
    fn strong_followed_by_backtick() {
        assert!(matches!(check_head_symbol("**`"), RawStyle::Strong));
    }
    #[test]
    fn asterisk_italic_followed_by_underscore() {
        assert!(matches!(check_head_symbol("*_"), RawStyle::AsteriskItalic));
    }
    #[test]
    fn asterisk_italic_followed_by_backtick() {
        assert!(matches!(check_head_symbol("*`"), RawStyle::AsteriskItalic));
    }
    #[test]
    fn underscore_italic_followed_by_asterisk() {
        assert!(matches!(
            check_head_symbol("_*"),
            RawStyle::UnderscoreItalic
        ));
    }
    #[test]
    fn underscore_italic_followed_by_backtick() {
        assert!(matches!(
            check_head_symbol("_`"),
            RawStyle::UnderscoreItalic
        ));
    }
    #[test]
    fn code_followed_by_asterisk() {
        assert!(matches!(check_head_symbol("`*"), RawStyle::Code));
    }
    #[test]
    fn code_followed_by_underscore() {
        assert!(matches!(check_head_symbol("`_"), RawStyle::Code));
    }
}
