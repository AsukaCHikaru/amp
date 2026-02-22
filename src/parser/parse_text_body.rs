use crate::{
    parser::{
        parse_link::{parse_link_in_text, LinkParseItem},
        parse_text_body_style::{merge_same_type_text_body, parse_text_body_style},
    },
    types::InlineContent,
};

pub fn parse_text_body(input: &str) -> Vec<InlineContent> {
    let link_parsed = parse_link_in_text(input);

    link_parsed
        .into_iter()
        .flat_map(|item| match item {
            LinkParseItem::Link(link) => vec![InlineContent::Link(link)],
            LinkParseItem::Text(text) => merge_same_type_text_body(parse_text_body_style(&text))
                .into_iter()
                .map(|tb| InlineContent::TextBody(tb))
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn tb(style: TextBodyStyle, value: &str) -> InlineContent {
        InlineContent::TextBody(TextBody {
            style,
            value: value.to_string(),
        })
    }

    fn plain(value: &str) -> InlineContent {
        tb(TextBodyStyle::Plain, value)
    }

    fn link(body: Vec<TextBody>, url: &str) -> InlineContent {
        InlineContent::Link(Link {
            body,
            url: url.to_string(),
        })
    }

    fn plain_tb(value: &str) -> TextBody {
        TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        }
    }

    mod plain_text_tests {
        use super::*;

        #[test]
        fn full_plain_text() {
            assert_eq!(
                parse_text_body("This is a plain text paragraph"),
                vec![plain("This is a plain text paragraph")]
            );
        }
    }

    mod strong_text_tests {
        use super::*;

        #[test]
        fn strong_only() {
            assert_eq!(
                parse_text_body("**strong**"),
                vec![tb(TextBodyStyle::Strong, "strong")]
            );
        }
        #[test]
        fn strong_at_start() {
            assert_eq!(
                parse_text_body("**strong** text follows"),
                vec![tb(TextBodyStyle::Strong, "strong"), plain(" text follows"),]
            );
        }
        #[test]
        fn strong_in_middle() {
            assert_eq!(
                parse_text_body("text with **strong** in the middle"),
                vec![
                    plain("text with "),
                    tb(TextBodyStyle::Strong, "strong"),
                    plain(" in the middle"),
                ]
            );
        }
        #[test]
        fn strong_at_end() {
            assert_eq!(
                parse_text_body("text ends with **strong**"),
                vec![
                    plain("text ends with "),
                    tb(TextBodyStyle::Strong, "strong"),
                ]
            );
        }
    }

    mod asterisk_italic_tests {
        use super::*;

        #[test]
        fn italic_only() {
            assert_eq!(
                parse_text_body("*italic*"),
                vec![tb(TextBodyStyle::Italic, "italic")]
            );
        }
        #[test]
        fn italic_at_start() {
            assert_eq!(
                parse_text_body("*italic* text follows"),
                vec![tb(TextBodyStyle::Italic, "italic"), plain(" text follows"),]
            );
        }
        #[test]
        fn italic_in_middle() {
            assert_eq!(
                parse_text_body("text with *italic* in the middle"),
                vec![
                    plain("text with "),
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" in the middle"),
                ]
            );
        }
        #[test]
        fn italic_at_end() {
            assert_eq!(
                parse_text_body("text ends with *italic*"),
                vec![
                    plain("text ends with "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
    }

    mod underscore_italic_tests {
        use super::*;

        #[test]
        fn italic_only() {
            assert_eq!(
                parse_text_body("_italic_"),
                vec![tb(TextBodyStyle::Italic, "italic")]
            );
        }
        #[test]
        fn italic_at_start() {
            assert_eq!(
                parse_text_body("_italic_ text follows"),
                vec![tb(TextBodyStyle::Italic, "italic"), plain(" text follows"),]
            );
        }
        #[test]
        fn italic_in_middle() {
            assert_eq!(
                parse_text_body("text with _italic_ in the middle"),
                vec![
                    plain("text with "),
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" in the middle"),
                ]
            );
        }
        #[test]
        fn italic_at_end() {
            assert_eq!(
                parse_text_body("text ends with _italic_"),
                vec![
                    plain("text ends with "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn link_text_with_underscores_not_parsed_as_italic() {
            assert_eq!(
                parse_text_body("_before_ [link_underscore](https://example.com) _after_"),
                vec![
                    tb(TextBodyStyle::Italic, "before"),
                    plain(" "),
                    link(vec![plain_tb("link_underscore")], "https://example.com"),
                    plain(" "),
                    tb(TextBodyStyle::Italic, "after"),
                ]
            );
        }
        #[test]
        fn link_url_with_underscores_not_parsed_as_italic() {
            assert_eq!(
                parse_text_body("_before_ [link](https://example.com/this_is_a_link) _after_"),
                vec![
                    tb(TextBodyStyle::Italic, "before"),
                    plain(" "),
                    link(vec![plain_tb("link")], "https://example.com/this_is_a_link"),
                    plain(" "),
                    tb(TextBodyStyle::Italic, "after"),
                ]
            );
        }
    }

    mod code_text_tests {
        use super::*;

        #[test]
        fn code_only() {
            assert_eq!(
                parse_text_body("`code`"),
                vec![tb(TextBodyStyle::Code, "code")]
            );
        }
        #[test]
        fn code_at_start() {
            assert_eq!(
                parse_text_body("`code` text follows"),
                vec![tb(TextBodyStyle::Code, "code"), plain(" text follows"),]
            );
        }
        #[test]
        fn code_in_middle() {
            assert_eq!(
                parse_text_body("text with `code` in the middle"),
                vec![
                    plain("text with "),
                    tb(TextBodyStyle::Code, "code"),
                    plain(" in the middle"),
                ]
            );
        }
        #[test]
        fn code_at_end() {
            assert_eq!(
                parse_text_body("text ends with `code`"),
                vec![plain("text ends with "), tb(TextBodyStyle::Code, "code"),]
            );
        }
    }

    mod long_styled_text_tests {
        use super::*;

        #[test]
        fn long_strong() {
            assert_eq!(
                parse_text_body("**multiple words in strong text**"),
                vec![tb(TextBodyStyle::Strong, "multiple words in strong text")]
            );
        }
        #[test]
        fn long_asterisk_italic() {
            assert_eq!(
                parse_text_body("*multiple words in italic text*"),
                vec![tb(TextBodyStyle::Italic, "multiple words in italic text")]
            );
        }
        #[test]
        fn long_underscore_italic() {
            assert_eq!(
                parse_text_body("_multiple words in italic text_"),
                vec![tb(TextBodyStyle::Italic, "multiple words in italic text")]
            );
        }
        #[test]
        fn long_code() {
            assert_eq!(
                parse_text_body("`const x = function() { return true; }`"),
                vec![tb(
                    TextBodyStyle::Code,
                    "const x = function() { return true; }"
                )]
            );
        }
    }

    mod mixed_styles_tests {
        use super::*;

        #[test]
        fn all_styles_mixed() {
            assert_eq!(
                parse_text_body("Plain **strong** and *italic* and `code` text"),
                vec![
                    plain("Plain "),
                    tb(TextBodyStyle::Strong, "strong"),
                    plain(" and "),
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" and "),
                    tb(TextBodyStyle::Code, "code"),
                    plain(" text"),
                ]
            );
        }
        #[test]
        fn complex_mixed_styles() {
            assert_eq!(
                parse_text_body("**Strong** at start, *italic* in middle, and `code` at the end"),
                vec![
                    tb(TextBodyStyle::Strong, "Strong"),
                    plain(" at start, "),
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" in middle, and "),
                    tb(TextBodyStyle::Code, "code"),
                    plain(" at the end"),
                ]
            );
        }
    }

    mod link_tests {
        use super::*;

        #[test]
        fn link_only() {
            assert_eq!(
                parse_text_body("[link text](https://example.com)"),
                vec![link(vec![plain_tb("link text")], "https://example.com")]
            );
        }
        #[test]
        fn link_at_start() {
            assert_eq!(
                parse_text_body("[link text](https://example.com) followed by text"),
                vec![
                    link(vec![plain_tb("link text")], "https://example.com"),
                    plain(" followed by text"),
                ]
            );
        }
        #[test]
        fn link_in_middle() {
            assert_eq!(
                parse_text_body("Text before [link text](https://example.com) and after"),
                vec![
                    plain("Text before "),
                    link(vec![plain_tb("link text")], "https://example.com"),
                    plain(" and after"),
                ]
            );
        }
        #[test]
        fn link_at_end() {
            assert_eq!(
                parse_text_body("Text ending with [link text](https://example.com)"),
                vec![
                    plain("Text ending with "),
                    link(vec![plain_tb("link text")], "https://example.com"),
                ]
            );
        }
        #[test]
        fn multiple_links() {
            assert_eq!(
                parse_text_body(
                    "[first link](https://example.com) and [second link](https://example.org)"
                ),
                vec![
                    link(vec![plain_tb("first link")], "https://example.com"),
                    plain(" and "),
                    link(vec![plain_tb("second link")], "https://example.org"),
                ]
            );
        }
        #[test]
        fn links_with_styled_text() {
            assert_eq!(
                parse_text_body(
                    "Text with **strong** and [link text](https://example.com) and *italic*"
                ),
                vec![
                    plain("Text with "),
                    tb(TextBodyStyle::Strong, "strong"),
                    plain(" and "),
                    link(vec![plain_tb("link text")], "https://example.com"),
                    plain(" and "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
    }

    mod inline_code_edge_cases {
        use super::*;

        #[test]
        fn code_with_underscores_inside() {
            assert_eq!(
                parse_text_body("`text_with_underscore`"),
                vec![tb(TextBodyStyle::Code, "text_with_underscore")]
            );
        }
        #[test]
        fn code_with_asterisks_inside() {
            assert_eq!(
                parse_text_body("`text*with*asterisks`"),
                vec![tb(TextBodyStyle::Code, "text*with*asterisks")]
            );
        }
        #[test]
        fn code_with_double_asterisks_not_bold() {
            assert_eq!(
                parse_text_body("`**not bold**`"),
                vec![tb(TextBodyStyle::Code, "**not bold**")]
            );
        }
        #[test]
        fn code_with_underscores_not_italic() {
            assert_eq!(
                parse_text_body("`_not italic_`"),
                vec![tb(TextBodyStyle::Code, "_not italic_")]
            );
        }
        #[test]
        fn code_with_double_underscores() {
            assert_eq!(
                parse_text_body("`__double_underscore__`"),
                vec![tb(TextBodyStyle::Code, "__double_underscore__")]
            );
        }
        #[test]
        fn code_between_italic_underscores() {
            assert_eq!(
                parse_text_body("_italic_ `code` _italic_"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" "),
                    tb(TextBodyStyle::Code, "code"),
                    plain(" "),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn code_between_strong() {
            assert_eq!(
                parse_text_body("**bold** `code` **bold**"),
                vec![
                    tb(TextBodyStyle::Strong, "bold"),
                    plain(" "),
                    tb(TextBodyStyle::Code, "code"),
                    plain(" "),
                    tb(TextBodyStyle::Strong, "bold"),
                ]
            );
        }
        #[test]
        fn code_adjacent_to_italic_no_spaces() {
            assert_eq!(
                parse_text_body("*italic*`code`*italic*"),
                vec![
                    tb(TextBodyStyle::Italic, "italic"),
                    tb(TextBodyStyle::Code, "code"),
                    tb(TextBodyStyle::Italic, "italic"),
                ]
            );
        }
        #[test]
        fn code_with_html_tags() {
            assert_eq!(
                parse_text_body("`<div>`"),
                vec![tb(TextBodyStyle::Code, "<div>")]
            );
        }
        #[test]
        fn code_with_object_syntax() {
            assert_eq!(
                parse_text_body("`{ key: value }`"),
                vec![tb(TextBodyStyle::Code, "{ key: value }")]
            );
        }
        #[test]
        fn multiple_code_spans_with_italic_between() {
            assert_eq!(
                parse_text_body("`code1` _italic_ `code2`"),
                vec![
                    tb(TextBodyStyle::Code, "code1"),
                    plain(" "),
                    tb(TextBodyStyle::Italic, "italic"),
                    plain(" "),
                    tb(TextBodyStyle::Code, "code2"),
                ]
            );
        }
        #[test]
        fn multiple_code_spans_with_plain_and_italic() {
            assert_eq!(
                parse_text_body("`a` and `b` with _c_"),
                vec![
                    tb(TextBodyStyle::Code, "a"),
                    plain(" and "),
                    tb(TextBodyStyle::Code, "b"),
                    plain(" with "),
                    tb(TextBodyStyle::Italic, "c"),
                ]
            );
        }
        #[test]
        fn code_with_single_space() {
            assert_eq!(parse_text_body("` `"), vec![tb(TextBodyStyle::Code, " ")]);
        }
        #[test]
        fn code_with_multiple_spaces() {
            assert_eq!(
                parse_text_body("`code with   spaces`"),
                vec![tb(TextBodyStyle::Code, "code with   spaces")]
            );
        }
        #[test]
        fn code_with_underscores_surrounded_by_italic() {
            assert_eq!(
                parse_text_body("Use _emphasis_ with `snake_case_variable` in _context_"),
                vec![
                    plain("Use "),
                    tb(TextBodyStyle::Italic, "emphasis"),
                    plain(" with "),
                    tb(TextBodyStyle::Code, "snake_case_variable"),
                    plain(" in "),
                    tb(TextBodyStyle::Italic, "context"),
                ]
            );
        }
    }
}
