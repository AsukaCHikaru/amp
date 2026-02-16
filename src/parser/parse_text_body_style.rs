use crate::types::{TextBody, TextBodyStyle};

enum RawStyle {
    Plain,
    Strong,
    AsteriskItalic,
    UnderscoreItalic,
    Code,
}

fn check_head_symbol(input: &str) -> RawStyle {
    match input.as_bytes() {
        [b'*', b'*', ..] => RawStyle::Strong,
        [b'*', ..] => RawStyle::AsteriskItalic,
        [b'_', ..] => RawStyle::UnderscoreItalic,
        [b'`', ..] => RawStyle::Code,
        _ => RawStyle::Plain,
    }
}

fn convert_raw_style_to_text_body_style(raw: RawStyle) -> TextBodyStyle {
    match raw {
        RawStyle::Strong => TextBodyStyle::Strong,
        RawStyle::Code => TextBodyStyle::Code,
        RawStyle::AsteriskItalic | RawStyle::UnderscoreItalic => TextBodyStyle::Italic,
        RawStyle::Plain => TextBodyStyle::Plain,
    }
}

struct LookupResult {
    text_body: TextBody,
    rest: String,
}

fn lookup_until_close(input: &str, style: &RawStyle) -> LookupResult {
    // placeholder
    LookupResult {
        text_body: TextBody {
            style: TextBodyStyle::Plain,
            value: "".to_string(),
        },
        rest: "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod check_head_symbol_tests {
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

    mod lookup_until_close_tests {
        use super::*;

        // Strong: closed
        #[test]
        fn strong_closed() {
            let result = lookup_until_close("**bold**", &RawStyle::Strong);
            assert!(matches!(result.text_body.style, TextBodyStyle::Strong));
            assert_eq!(result.text_body.value, "bold");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn strong_closed_with_rest() {
            let result = lookup_until_close("**bold** rest text", &RawStyle::Strong);
            assert!(matches!(result.text_body.style, TextBodyStyle::Strong));
            assert_eq!(result.text_body.value, "bold");
            assert_eq!(result.rest, " rest text");
        }
        #[test]
        fn strong_closed_multiword() {
            let result = lookup_until_close("**multiple words**", &RawStyle::Strong);
            assert!(matches!(result.text_body.style, TextBodyStyle::Strong));
            assert_eq!(result.text_body.value, "multiple words");
            assert_eq!(result.rest, "");
        }

        // Strong: unclosed
        #[test]
        fn strong_unclosed_at_end() {
            let result = lookup_until_close("**unclosed", &RawStyle::Strong);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "**unclosed");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn strong_unclosed_cut_at_next_marker() {
            let result = lookup_until_close("**unclosed _next", &RawStyle::Strong);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "**unclosed ");
            assert_eq!(result.rest, "_next");
        }

        // Asterisk italic: closed
        #[test]
        fn asterisk_italic_closed() {
            let result = lookup_until_close("*italic*", &RawStyle::AsteriskItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Italic));
            assert_eq!(result.text_body.value, "italic");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn asterisk_italic_closed_with_rest() {
            let result = lookup_until_close("*italic* rest", &RawStyle::AsteriskItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Italic));
            assert_eq!(result.text_body.value, "italic");
            assert_eq!(result.rest, " rest");
        }

        // Asterisk italic: unclosed
        #[test]
        fn asterisk_italic_unclosed_at_end() {
            let result = lookup_until_close("*unclosed", &RawStyle::AsteriskItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "*unclosed");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn asterisk_italic_unclosed_cut_at_next_marker() {
            let result = lookup_until_close("*unclosed `next", &RawStyle::AsteriskItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "*unclosed ");
            assert_eq!(result.rest, "`next");
        }

        // Underscore italic: closed
        #[test]
        fn underscore_italic_closed() {
            let result = lookup_until_close("_italic_", &RawStyle::UnderscoreItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Italic));
            assert_eq!(result.text_body.value, "italic");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn underscore_italic_closed_with_rest() {
            let result = lookup_until_close("_italic_ rest", &RawStyle::UnderscoreItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Italic));
            assert_eq!(result.text_body.value, "italic");
            assert_eq!(result.rest, " rest");
        }

        // Underscore italic: unclosed
        #[test]
        fn underscore_italic_unclosed_at_end() {
            let result = lookup_until_close("_unclosed", &RawStyle::UnderscoreItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "_unclosed");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn underscore_italic_unclosed_cut_at_next_marker() {
            let result = lookup_until_close("_unclosed **next", &RawStyle::UnderscoreItalic);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "_unclosed ");
            assert_eq!(result.rest, "**next");
        }

        // Code: closed
        #[test]
        fn code_closed() {
            let result = lookup_until_close("`code`", &RawStyle::Code);
            assert!(matches!(result.text_body.style, TextBodyStyle::Code));
            assert_eq!(result.text_body.value, "code");
            assert_eq!(result.rest, "");
        }
        #[test]
        fn code_closed_with_rest() {
            let result = lookup_until_close("`code` rest", &RawStyle::Code);
            assert!(matches!(result.text_body.style, TextBodyStyle::Code));
            assert_eq!(result.text_body.value, "code");
            assert_eq!(result.rest, " rest");
        }
        #[test]
        fn code_preserves_inner_markers() {
            let result = lookup_until_close("`**not bold**`", &RawStyle::Code);
            assert!(matches!(result.text_body.style, TextBodyStyle::Code));
            assert_eq!(result.text_body.value, "**not bold**");
            assert_eq!(result.rest, "");
        }

        // Code: unclosed
        #[test]
        fn code_unclosed_at_end() {
            let result = lookup_until_close("`unclosed", &RawStyle::Code);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "`unclosed");
            assert_eq!(result.rest, "");
        }

        // Plain
        #[test]
        fn plain_consumes_until_marker() {
            let result = lookup_until_close("plain text **bold", &RawStyle::Plain);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "plain text ");
            assert_eq!(result.rest, "**bold");
        }
        #[test]
        fn plain_consumes_all_when_no_marker() {
            let result = lookup_until_close("just plain text", &RawStyle::Plain);
            assert!(matches!(result.text_body.style, TextBodyStyle::Plain));
            assert_eq!(result.text_body.value, "just plain text");
            assert_eq!(result.rest, "");
        }
    }
}
