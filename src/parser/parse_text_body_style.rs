use std::{sync::LazyLock, vec};

use regex::Regex;

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

fn convert_raw_style_to_text_body_style(raw: &RawStyle) -> TextBodyStyle {
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

static STRONG_REGULAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\*{2}([^*_`]+)\*{2}([\s\S]*)").expect("Invalid regex"));
static ASTERISK_ITALIC_REGULAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\*{1}([^*_`]+)\*{1}([\s\S]*)").expect("Invalid regex"));
static UNDERSCORE_ITALIC_REGULAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^_{1}([^*_`]+)_{1}([\s\S]*)").expect("Invalid regex"));
static CODE_REGULAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^`([^`]+)`([\s\S]*)").expect("Invalid regex"));
static PLAIN_REGULAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([^*_`]+)([\s\S]*)$").expect("Invalid regex"));

static STRONG_UNCLOSED_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\*{2}[^*_`]+?)([*_`][\s\S]+)*$").expect("Invalid regex"));
static ASTERISK_ITALIC_UNCLOSED_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\*{1}[^*_`]+?)([*_`][\s\S]+)*$").expect("Invalid regex"));
static UNDERSCORE_ITALIC_UNCLOSED_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(_{1}[^*_`]+?)([*_`][\s\S]+)*$").expect("Invalid regex"));
static CODE_UNCLOSED_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(`[^`]+)$").expect("Invalid regex"));

fn lookup_until_close(input: &str, style: &RawStyle) -> LookupResult {
    let regular_pattern = match style {
        RawStyle::Strong => &STRONG_REGULAR_PATTERN,
        RawStyle::AsteriskItalic => &ASTERISK_ITALIC_REGULAR_PATTERN,
        RawStyle::UnderscoreItalic => &UNDERSCORE_ITALIC_REGULAR_PATTERN,
        RawStyle::Code => &CODE_REGULAR_PATTERN,
        RawStyle::Plain => &PLAIN_REGULAR_PATTERN,
    };
    match regular_pattern.captures(input) {
        Some(reg_cap) => {
            let value = reg_cap.get(1).unwrap().as_str().to_string();
            let rest = reg_cap.get(2).unwrap().as_str().to_string();
            LookupResult {
                text_body: TextBody {
                    style: convert_raw_style_to_text_body_style(&style),
                    value,
                },
                rest,
            }
        }
        None => {
            let unclosed_pattern = match style {
                RawStyle::Strong => &STRONG_UNCLOSED_PATTERN,
                RawStyle::AsteriskItalic => &ASTERISK_ITALIC_UNCLOSED_PATTERN,
                RawStyle::UnderscoreItalic => &UNDERSCORE_ITALIC_UNCLOSED_PATTERN,
                RawStyle::Code => &CODE_UNCLOSED_PATTERN,
                RawStyle::Plain => unreachable!("Regular pattern always matches"),
            };
            match unclosed_pattern.captures(input) {
                Some(unclosed_cap) => {
                    let value = unclosed_cap.get(1).unwrap().as_str().to_string();
                    let rest = unclosed_cap
                        .get(2)
                        .map(|m| m.as_str())
                        .unwrap_or("")
                        .to_string();
                    LookupResult {
                        text_body: TextBody {
                            style: TextBodyStyle::Plain,
                            value,
                        },
                        rest,
                    }
                }
                None => unreachable!("Regular pattern doesn't have unclosed match"),
            }
        }
    }
}

fn parse_text_body_style(input: &str) -> Vec<TextBody> {
    if input.is_empty() {
        return vec![];
    }
    let head_style = check_head_symbol(input);
    let LookupResult { text_body, rest } = lookup_until_close(input, &head_style);

    let mut result = vec![text_body];
    result.extend(parse_text_body_style(rest.as_str()));
    result
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

    mod parse_text_body_style_tests {
        use super::*;

        fn tb(style: TextBodyStyle, value: &str) -> TextBody {
            TextBody {
                style,
                value: value.to_string(),
            }
        }

        // Empty input
        #[test]
        fn empty_string_returns_empty_vec() {
            assert_eq!(parse_text_body_style(""), vec![]);
        }

        // Plain
        #[test]
        fn plain() {
            assert_eq!(
                parse_text_body_style("plaintext"),
                vec![tb(TextBodyStyle::Plain, "plaintext")]
            );
        }
        #[test]
        fn plain_with_space() {
            assert_eq!(
                parse_text_body_style("plain text"),
                vec![tb(TextBodyStyle::Plain, "plain text")]
            );
        }

        // Strong
        #[test]
        fn strong_only() {
            assert_eq!(
                parse_text_body_style("**strong**"),
                vec![tb(TextBodyStyle::Strong, "strong")]
            );
        }
        #[test]
        fn strong_at_start() {
            assert_eq!(
                parse_text_body_style("**strong** text follows"),
                vec![
                    tb(TextBodyStyle::Strong, "strong"),
                    tb(TextBodyStyle::Plain, " text follows"),
                ]
            );
        }
        #[test]
        fn strong_in_middle() {
            assert_eq!(
                parse_text_body_style("text with **strong** in the middle"),
                vec![
                    tb(TextBodyStyle::Plain, "text with "),
                    tb(TextBodyStyle::Strong, "strong"),
                    tb(TextBodyStyle::Plain, " in the middle"),
                ]
            );
        }
        #[test]
        fn strong_at_end() {
            assert_eq!(
                parse_text_body_style("text ends with **strong**"),
                vec![
                    tb(TextBodyStyle::Plain, "text ends with "),
                    tb(TextBodyStyle::Strong, "strong"),
                ]
            );
        }
        #[test]
        fn strong_with_multiple_words() {
            assert_eq!(
                parse_text_body_style("**multiple words in strong**"),
                vec![tb(TextBodyStyle::Strong, "multiple words in strong")]
            );
        }

        // Italic (asterisks)
        #[test]
        fn asterisk_italic_only() {
            assert_eq!(
                parse_text_body_style("*italic*"),
                vec![tb(TextBodyStyle::Italic, "italic")]
            );
        }
        #[test]
        fn asterisk_italic_at_start() {
            assert_eq!(
                parse_text_body_style("*italic* text follows"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " text follows"),
                ]
            );
        }
        #[test]
        fn asterisk_italic_in_middle() {
            assert_eq!(
                parse_text_body_style("text with *italic* in the middle"),
                vec![
                    tb(TextBodyStyle::Plain, "text with "),
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " in the middle"),
                ]
            );
        }
        #[test]
        fn asterisk_italic_at_end() {
            assert_eq!(
                parse_text_body_style("text ends with *italic*"),
                vec![
                    tb(TextBodyStyle::Plain, "text ends with "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn asterisk_italic_with_multiple_words() {
            assert_eq!(
                parse_text_body_style("*multiple words in italic*"),
                vec![tb(TextBodyStyle::Italic, "multiple words in italic")]
            );
        }

        // Italic (underscores)
        #[test]
        fn underscore_italic_only() {
            assert_eq!(
                parse_text_body_style("_italic_"),
                vec![tb(TextBodyStyle::Italic, "italic")]
            );
        }
        #[test]
        fn underscore_italic_at_start() {
            assert_eq!(
                parse_text_body_style("_italic_ text follows"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " text follows"),
                ]
            );
        }
        #[test]
        fn underscore_italic_in_middle() {
            assert_eq!(
                parse_text_body_style("text with _italic_ in the middle"),
                vec![
                    tb(TextBodyStyle::Plain, "text with "),
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " in the middle"),
                ]
            );
        }
        #[test]
        fn underscore_italic_at_end() {
            assert_eq!(
                parse_text_body_style("text ends with _italic_"),
                vec![
                    tb(TextBodyStyle::Plain, "text ends with "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn underscore_italic_with_multiple_words() {
            assert_eq!(
                parse_text_body_style("_multiple words in italic_"),
                vec![tb(TextBodyStyle::Italic, "multiple words in italic")]
            );
        }

        // Code
        #[test]
        fn code_only() {
            assert_eq!(
                parse_text_body_style("`code`"),
                vec![tb(TextBodyStyle::Code, "code")]
            );
        }
        #[test]
        fn code_at_start() {
            assert_eq!(
                parse_text_body_style("`code` text follows"),
                vec![
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " text follows"),
                ]
            );
        }
        #[test]
        fn code_in_middle() {
            assert_eq!(
                parse_text_body_style("text with `code` in the middle"),
                vec![
                    tb(TextBodyStyle::Plain, "text with "),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " in the middle"),
                ]
            );
        }
        #[test]
        fn code_at_end() {
            assert_eq!(
                parse_text_body_style("text ends with `code`"),
                vec![
                    tb(TextBodyStyle::Plain, "text ends with "),
                    tb(TextBodyStyle::Code, "code"),
                ]
            );
        }
        #[test]
        fn code_with_multiple_words() {
            assert_eq!(
                parse_text_body_style("`const x = function() { return true; }`"),
                vec![tb(
                    TextBodyStyle::Code,
                    "const x = function() { return true; }"
                )]
            );
        }

        // Inline code edge cases
        #[test]
        fn code_with_underscores_inside() {
            assert_eq!(
                parse_text_body_style("`text_with_underscore`"),
                vec![tb(TextBodyStyle::Code, "text_with_underscore")]
            );
        }
        #[test]
        fn code_with_asterisks_inside() {
            assert_eq!(
                parse_text_body_style("`text*with*asterisks`"),
                vec![tb(TextBodyStyle::Code, "text*with*asterisks")]
            );
        }
        #[test]
        fn code_with_double_asterisks_inside_not_bold() {
            assert_eq!(
                parse_text_body_style("`**not bold**`"),
                vec![tb(TextBodyStyle::Code, "**not bold**")]
            );
        }
        #[test]
        fn code_with_underscores_inside_not_italic() {
            assert_eq!(
                parse_text_body_style("`_not italic_`"),
                vec![tb(TextBodyStyle::Code, "_not italic_")]
            );
        }
        #[test]
        fn code_with_html_tags() {
            assert_eq!(
                parse_text_body_style("`<div>`"),
                vec![tb(TextBodyStyle::Code, "<div>")]
            );
        }
        #[test]
        fn code_with_single_space() {
            assert_eq!(
                parse_text_body_style("` `"),
                vec![tb(TextBodyStyle::Code, " ")]
            );
        }
        #[test]
        fn code_with_multiple_spaces() {
            assert_eq!(
                parse_text_body_style("`code with   spaces`"),
                vec![tb(TextBodyStyle::Code, "code with   spaces")]
            );
        }

        // Mixed styles
        #[test]
        fn all_styles_mixed() {
            assert_eq!(
                parse_text_body_style("Plain **strong** and *italic* and `code` text"),
                vec![
                    tb(TextBodyStyle::Plain, "Plain "),
                    tb(TextBodyStyle::Strong, "strong"),
                    tb(TextBodyStyle::Plain, " and "),
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " and "),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " text"),
                ]
            );
        }
        #[test]
        fn styled_at_start_middle_and_end() {
            assert_eq!(
                parse_text_body_style(
                    "**Strong** at start, *italic* in middle, and `code` at the end"
                ),
                vec![
                    tb(TextBodyStyle::Strong, "Strong"),
                    tb(TextBodyStyle::Plain, " at start, "),
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " in middle, and "),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " at the end"),
                ]
            );
        }
        #[test]
        fn code_between_italic_underscores() {
            assert_eq!(
                parse_text_body_style("_italic_ `code` _italic_"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Plain, " "),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn code_between_strong() {
            assert_eq!(
                parse_text_body_style("**bold** `code` **bold**"),
                vec![
                    tb(TextBodyStyle::Strong, "bold"),
                    tb(TextBodyStyle::Plain, " "),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Plain, " "),
                    tb(TextBodyStyle::Strong, "bold"),
                ]
            );
        }
        #[test]
        fn code_adjacent_to_italic_without_spaces() {
            assert_eq!(
                parse_text_body_style("*italic*`code`*italic*"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn multiple_code_spans_with_plain_and_italic() {
            assert_eq!(
                parse_text_body_style("`a` and `b` with _c_"),
                vec![
                    tb(TextBodyStyle::Code, "a"),
                    tb(TextBodyStyle::Plain, " and "),
                    tb(TextBodyStyle::Code, "b"),
                    tb(TextBodyStyle::Plain, " with "),
                    tb(TextBodyStyle::Italic, "c"),
                ]
            );
        }
        #[test]
        fn code_with_underscores_surrounded_by_italic() {
            assert_eq!(
                parse_text_body_style("Use _emphasis_ with `snake_case_variable` in _context_"),
                vec![
                    tb(TextBodyStyle::Plain, "Use "),
                    tb(TextBodyStyle::Italic, "emphasis"),
                    tb(TextBodyStyle::Plain, " with "),
                    tb(TextBodyStyle::Code, "snake_case_variable"),
                    tb(TextBodyStyle::Plain, " in "),
                    tb(TextBodyStyle::Italic, "context"),
                ]
            );
        }
        #[test]
        fn code_with_mixed_markdown_syntax_inside() {
            assert_eq!(
                parse_text_body_style("`**bold** and _italic_ and [link](url)`"),
                vec![tb(
                    TextBodyStyle::Code,
                    "**bold** and _italic_ and [link](url)"
                )]
            );
        }

        // Unclosed markers
        #[test]
        fn unclosed_strong_at_end() {
            assert_eq!(
                parse_text_body_style("text **unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "text "),
                    tb(TextBodyStyle::Plain, "**unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_asterisk_italic_at_end() {
            assert_eq!(
                parse_text_body_style("text *unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "text "),
                    tb(TextBodyStyle::Plain, "*unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_underscore_italic_at_end() {
            assert_eq!(
                parse_text_body_style("text _unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "text "),
                    tb(TextBodyStyle::Plain, "_unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_code_at_end() {
            assert_eq!(
                parse_text_body_style("text `unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "text "),
                    tb(TextBodyStyle::Plain, "`unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_marker_at_very_start() {
            assert_eq!(
                parse_text_body_style("**unclosed"),
                vec![tb(TextBodyStyle::Plain, "**unclosed")]
            );
        }
        #[test]
        fn unclosed_strong_cut_at_next_unclosed_italic() {
            assert_eq!(
                parse_text_body_style("**unclosed _also unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "**unclosed "),
                    tb(TextBodyStyle::Plain, "_also unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_italic_cut_at_next_unclosed_code() {
            assert_eq!(
                parse_text_body_style("*unclosed `also unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "*unclosed "),
                    tb(TextBodyStyle::Plain, "`also unclosed"),
                ]
            );
        }
        #[test]
        fn three_different_unclosed_markers() {
            assert_eq!(
                parse_text_body_style("_one **two `three"),
                vec![
                    tb(TextBodyStyle::Plain, "_one "),
                    tb(TextBodyStyle::Plain, "**two "),
                    tb(TextBodyStyle::Plain, "`three"),
                ]
            );
        }
        #[test]
        fn closed_style_followed_by_unclosed_marker() {
            assert_eq!(
                parse_text_body_style("**bold** then _unclosed"),
                vec![
                    tb(TextBodyStyle::Strong, "bold"),
                    tb(TextBodyStyle::Plain, " then "),
                    tb(TextBodyStyle::Plain, "_unclosed"),
                ]
            );
        }
        #[test]
        fn unclosed_marker_followed_by_closed_style() {
            assert_eq!(
                parse_text_body_style("_unclosed **bold**"),
                vec![
                    tb(TextBodyStyle::Plain, "_unclosed "),
                    tb(TextBodyStyle::Strong, "bold"),
                ]
            );
        }
        #[test]
        fn unclosed_underscore_cut_at_next_unclosed_strong() {
            assert_eq!(
                parse_text_body_style("_unclosed **strong"),
                vec![
                    tb(TextBodyStyle::Plain, "_unclosed "),
                    tb(TextBodyStyle::Plain, "**strong"),
                ]
            );
        }
        #[test]
        fn closed_style_between_two_unclosed_markers() {
            assert_eq!(
                parse_text_body_style("_unclosed **bold** `also unclosed"),
                vec![
                    tb(TextBodyStyle::Plain, "_unclosed "),
                    tb(TextBodyStyle::Strong, "bold"),
                    tb(TextBodyStyle::Plain, " "),
                    tb(TextBodyStyle::Plain, "`also unclosed"),
                ]
            );
        }
    }
}
