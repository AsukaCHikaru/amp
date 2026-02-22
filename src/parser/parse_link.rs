use std::sync::LazyLock;

use regex::Regex;

use crate::{
    parser::parse_text_body_style::{merge_same_type_text_body, parse_text_body_style},
    types::{Link, TextBody, TextBodyStyle},
};

static LINK_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(.+?)\]\((.+?)\)").expect("Invalid regex"));

pub enum LinkParseItem {
    Text(String),
    Link(Link),
}

pub fn parse_link_in_text(input: &str) -> Vec<LinkParseItem> {
    match LINK_PATTERN.captures(input) {
        Some(captured) => {
            let text_cap = captured.get(1).unwrap();
            let url_cap = captured.get(2).unwrap();
            let body =
                merge_same_type_text_body(parse_text_body_style(&text_cap.as_str().to_string()));
            let link_block: Link = Link {
                body,
                url: url_cap.as_str().to_string(),
            };
            let before_cap = input[..text_cap.start() - 1].to_string();
            let after_cap = &input[(url_cap.end() + 1)..];
            println!("{}, {}", before_cap, after_cap);

            let mut result: Vec<LinkParseItem> = vec![LinkParseItem::Link(link_block)];
            if !before_cap.is_empty() {
                result.insert(0, LinkParseItem::Text(before_cap));
            }
            result.extend(parse_link_in_text(after_cap));

            result
        }
        None => {
            let mut result: Vec<LinkParseItem> = vec![];
            if !input.is_empty() {
                result.push(LinkParseItem::Text(input.to_string()));
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(s: &str) -> LinkParseItem {
        LinkParseItem::Text(s.to_string())
    }

    fn link(body: Vec<TextBody>, url: &str) -> LinkParseItem {
        LinkParseItem::Link(Link {
            body,
            url: url.to_string(),
        })
    }

    fn tb(style: TextBodyStyle, value: &str) -> TextBody {
        TextBody {
            style,
            value: value.to_string(),
        }
    }

    fn assert_parse_link(input: &str, expected: Vec<LinkParseItem>) {
        let result = parse_link_in_text(input);
        assert_eq!(
            result.len(),
            expected.len(),
            "length mismatch for: {}",
            input
        );
        for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
            match (r, e) {
                (LinkParseItem::Text(r_text), LinkParseItem::Text(e_text)) => {
                    assert_eq!(r_text, e_text, "text mismatch at index {}", i);
                }
                (LinkParseItem::Link(r_link), LinkParseItem::Link(e_link)) => {
                    assert_eq!(r_link, e_link, "link mismatch at index {}", i);
                }
                _ => panic!("variant mismatch at index {}: got different types", i),
            }
        }
    }

    mod parse_link_in_text_tests {
        use super::*;

        // No links
        #[test]
        fn no_links_returns_text() {
            assert_parse_link(
                "This is a plain text with no links",
                vec![text("This is a plain text with no links")],
            );
        }
        #[test]
        fn empty_string_returns_empty_text() {
            assert_parse_link("", vec![]);
        }

        // Single link
        #[test]
        fn single_link_only() {
            assert_parse_link(
                "[link text](https://example.com)",
                vec![link(
                    vec![tb(TextBodyStyle::Plain, "link text")],
                    "https://example.com",
                )],
            );
        }
        #[test]
        fn single_link_with_text_before() {
            assert_parse_link(
                "Text before [link text](https://example.com)",
                vec![
                    text("Text before "),
                    link(
                        vec![tb(TextBodyStyle::Plain, "link text")],
                        "https://example.com",
                    ),
                ],
            );
        }
        #[test]
        fn single_link_with_text_after() {
            assert_parse_link(
                "[link text](https://example.com) text after",
                vec![
                    link(
                        vec![tb(TextBodyStyle::Plain, "link text")],
                        "https://example.com",
                    ),
                    text(" text after"),
                ],
            );
        }
        #[test]
        fn single_link_with_text_before_and_after() {
            assert_parse_link(
                "text before [link text](https://example.com) text after",
                vec![
                    text("text before "),
                    link(
                        vec![tb(TextBodyStyle::Plain, "link text")],
                        "https://example.com",
                    ),
                    text(" text after"),
                ],
            );
        }

        // Multiple links
        #[test]
        fn multiple_links() {
            assert_parse_link(
                "[first link](https://example.com) and [second link](https://example.org)",
                vec![
                    link(
                        vec![tb(TextBodyStyle::Plain, "first link")],
                        "https://example.com",
                    ),
                    text(" and "),
                    link(
                        vec![tb(TextBodyStyle::Plain, "second link")],
                        "https://example.org",
                    ),
                ],
            );
        }
        #[test]
        fn multiple_links_with_text_before_between_and_after() {
            assert_parse_link(
                "start [first](https://example.com) middle [second](https://example.org) end",
                vec![
                    text("start "),
                    link(
                        vec![tb(TextBodyStyle::Plain, "first")],
                        "https://example.com",
                    ),
                    text(" middle "),
                    link(
                        vec![tb(TextBodyStyle::Plain, "second")],
                        "https://example.org",
                    ),
                    text(" end"),
                ],
            );
        }

        // Edge cases
        #[test]
        fn link_with_empty_text() {
            assert_parse_link(
                "[](https://example.com)",
                vec![text("[](https://example.com)")],
            );
        }
        #[test]
        fn link_with_empty_url() {
            assert_parse_link("[link text]()", vec![text("[link text]()")]);
        }
        #[test]
        fn link_with_special_characters_in_url() {
            assert_parse_link(
                "[link text](https://example.com?param=value&another=true)",
                vec![link(
                    vec![tb(TextBodyStyle::Plain, "link text")],
                    "https://example.com?param=value&another=true",
                )],
            );
        }
        #[test]
        fn link_with_special_characters_in_text() {
            assert_parse_link(
                "[link & text with special chars!](https://example.com)",
                vec![link(
                    vec![tb(TextBodyStyle::Plain, "link & text with special chars!")],
                    "https://example.com",
                )],
            );
        }

        // Complex cases
        #[test]
        fn adjacent_links() {
            assert_parse_link(
                "[first link](https://example.com)[second link](https://example.org)",
                vec![
                    link(
                        vec![tb(TextBodyStyle::Plain, "first link")],
                        "https://example.com",
                    ),
                    link(
                        vec![tb(TextBodyStyle::Plain, "second link")],
                        "https://example.org",
                    ),
                ],
            );
        }

        // Link body with styles
        #[test]
        fn link_with_bold_text() {
            assert_parse_link(
                "[**bold link**](https://example.com)",
                vec![link(
                    vec![tb(TextBodyStyle::Strong, "bold link")],
                    "https://example.com",
                )],
            );
        }
        #[test]
        fn link_with_italic_text() {
            assert_parse_link(
                "[*italic link*](https://example.com)",
                vec![link(
                    vec![tb(TextBodyStyle::Italic, "italic link")],
                    "https://example.com",
                )],
            );
        }
        #[test]
        fn link_with_code_text() {
            assert_parse_link(
                "[`code link`](https://example.com)",
                vec![link(
                    vec![tb(TextBodyStyle::Code, "code link")],
                    "https://example.com",
                )],
            );
        }

        // Malformed links
        #[test]
        fn ignores_malformed_link_without_closing_bracket() {
            let input = "Text with (https://example.com[malformed link";
            assert_parse_link(input, vec![text(input)]);
        }
        #[test]
        fn ignores_malformed_link_without_closing_parenthesis() {
            let input = "Text with (https://example.com[malformed link]";
            assert_parse_link(input, vec![text(input)]);
        }
    }
}
