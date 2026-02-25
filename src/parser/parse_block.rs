use std::sync::LazyLock;

use regex::Regex;

use crate::{
    parser::parse_text_body::parse_text_body,
    types::{
        Block, CodeBlock, HeadingBlock, HeadingLevel, ImageBlock, ListBlock, ListItem,
        ParagraphBlock, QuoteBlock, ThematicBreak,
    },
};

static PARAGRAPH_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([\s\S]+?)(?:\n|$)").unwrap());
static HEADING_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,6})\s(.+)").unwrap());
static IMAGE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^!\[(.*)\]\((.+?)\)(.*)").unwrap());
static CODE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^```(\w+)?\n([\s\S]*?)\n```").unwrap());
static LIST_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:(?:-|\d+\.)\s(.+)\n?)+").unwrap());
static QUOTE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(>\s.*\n?)+").unwrap());
static THEMATIC_BREAK_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(-{3,})").unwrap());

pub fn parse_blocks(input: &str) -> Vec<Block> {
    if input.trim().is_empty() {
        return vec![];
    }

    let mut result: Vec<Block> = vec![];

    if let Some(captured) = HEADING_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::Heading(parse_heading_block(input)));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = QUOTE_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::Quote(parse_quote_block(cap.as_str())));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = LIST_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::List(parse_list_block(cap.as_str())));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = IMAGE_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::Image(parse_image_block(cap.as_str())));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = CODE_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::Code(parse_code_block(cap.as_str())));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = THEMATIC_BREAK_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::ThematicBreak(ThematicBreak {}));
        result.extend(parse_blocks(&rest));
    } else if let Some(captured) = PARAGRAPH_PATTERN.captures(input) {
        let cap = captured.get(0).unwrap();
        let rest = input[cap.end()..].trim().to_string();
        result.push(Block::Paragraph(parse_paragraph_block(cap.as_str())));
        result.extend(parse_blocks(&rest));
    }

    result
}

fn parse_paragraph_block(input: &str) -> ParagraphBlock {
    ParagraphBlock {
        body: parse_text_body(input.trim()),
    }
}

fn parse_heading_block(input: &str) -> HeadingBlock {
    let cap = HEADING_PATTERN.captures(input).unwrap();
    let text = cap.get(2).unwrap().as_str();
    let level = cap.get(1).unwrap().as_str().len() as u8;
    HeadingBlock {
        body: parse_text_body(text),
        level: HeadingLevel::new(level).unwrap(),
    }
}

static TRIM_CAPTION_PARENTHESES_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\(([\s\S]+?)\)$").unwrap());
fn parse_image_block(input: &str) -> ImageBlock {
    let cap = IMAGE_PATTERN.captures(input).unwrap();
    let alt_text = cap.get(1).unwrap().as_str().to_string();
    let url = cap.get(2).unwrap().as_str().to_string();
    let caption = cap
        .get(3)
        .map(|m| {
            TRIM_CAPTION_PARENTHESES_PATTERN
                .replace(m.as_str(), "$1")
                .to_string()
        })
        .unwrap_or("".to_string());

    ImageBlock {
        url,
        alt_text,
        caption,
    }
}
fn parse_code_block(input: &str) -> CodeBlock {
    let cap = CODE_PATTERN.captures(input).unwrap();
    let lang = cap.get(1).map(|m| m.as_str().to_string());
    let text = cap.get(2).unwrap().as_str().to_string();
    CodeBlock { lang, body: text }
}

fn parse_list_item(input: &str) -> ListItem {
    let cap = LIST_PATTERN.captures(input).unwrap();
    let text = cap.get(1).unwrap().as_str();
    ListItem {
        body: parse_text_body(text),
    }
}
static LIST_ORDERED_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{1,}").unwrap());
fn parse_list_block(input: &str) -> ListBlock {
    let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let items = lines.iter().map(|line| parse_list_item(line)).collect();
    let ordered = lines.iter().all(|line| LIST_ORDERED_PATTERN.is_match(line));
    ListBlock {
        body: items,
        ordered,
    }
}

static QUOTE_STRIP_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n>[ ]?").expect("Invalid regex"));
static QUOTE_START_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^>\s").expect("Invalid regex"));
fn parse_quote_block(input: &str) -> QuoteBlock {
    let text = QUOTE_START_PATTERN
        .replace(
            &QUOTE_STRIP_PATTERN.replace_all(input, "\n").to_string(),
            "",
        )
        .trim()
        .to_string();

    QuoteBlock {
        body: parse_text_body(&text),
    }
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

    mod parse_image_block_tests {
        use super::*;

        #[test]
        fn basic_image_with_alt_url_and_caption() {
            assert_eq!(
                parse_image_block("![Sample Alt Text](/images/sample.jpg)(This is a caption)"),
                ImageBlock {
                    url: "/images/sample.jpg".to_string(),
                    alt_text: "Sample Alt Text".to_string(),
                    caption: "This is a caption".to_string(),
                }
            );
        }
        #[test]
        fn image_with_spaces_in_url() {
            assert_eq!(
                parse_image_block("![Alt text](/path/to/image with spaces.jpg)(Caption)"),
                ImageBlock {
                    url: "/path/to/image with spaces.jpg".to_string(),
                    alt_text: "Alt text".to_string(),
                    caption: "Caption".to_string(),
                }
            );
        }
        #[test]
        fn image_with_empty_alt_text() {
            assert_eq!(
                parse_image_block("![](/images/no-alt.jpg)(Image without alt text)"),
                ImageBlock {
                    url: "/images/no-alt.jpg".to_string(),
                    alt_text: "".to_string(),
                    caption: "Image without alt text".to_string(),
                }
            );
        }
        #[test]
        fn image_without_caption() {
            assert_eq!(
                parse_image_block("![Alt text only](/images/no-caption.jpg)"),
                ImageBlock {
                    url: "/images/no-caption.jpg".to_string(),
                    alt_text: "Alt text only".to_string(),
                    caption: "".to_string(),
                }
            );
        }
    }

    mod parse_heading_block_tests {
        use super::*;

        #[test]
        fn parses_h1() {
            assert_eq!(
                parse_heading_block("# Heading 1"),
                HeadingBlock {
                    level: HeadingLevel::new(1).unwrap(),
                    body: vec![plain("Heading 1")],
                }
            );
        }
        #[test]
        fn parses_h2() {
            assert_eq!(
                parse_heading_block("## Heading 2"),
                HeadingBlock {
                    level: HeadingLevel::new(2).unwrap(),
                    body: vec![plain("Heading 2")],
                }
            );
        }
        #[test]
        fn parses_h6() {
            assert_eq!(
                parse_heading_block("###### Heading 6"),
                HeadingBlock {
                    level: HeadingLevel::new(6).unwrap(),
                    body: vec![plain("Heading 6")],
                }
            );
        }
        #[test]
        fn heading_with_styled_text() {
            assert_eq!(
                parse_heading_block("# Heading with **strong** and *italic* text"),
                HeadingBlock {
                    level: HeadingLevel::new(1).unwrap(),
                    body: vec![
                        plain("Heading with "),
                        tb(TextBodyStyle::Strong, "strong"),
                        plain(" and "),
                        tb(TextBodyStyle::Italic, "italic"),
                        plain(" text"),
                    ],
                }
            );
        }
    }

    mod parse_quote_block_tests {
        use super::*;

        #[test]
        fn simple_quote() {
            assert_eq!(
                parse_quote_block("> This is a quote"),
                QuoteBlock {
                    body: vec![plain("This is a quote")],
                }
            );
        }
        #[test]
        fn quote_with_styled_text() {
            assert_eq!(
                parse_quote_block("> Quote with **strong** and *italic* text"),
                QuoteBlock {
                    body: vec![
                        plain("Quote with "),
                        tb(TextBodyStyle::Strong, "strong"),
                        plain(" and "),
                        tb(TextBodyStyle::Italic, "italic"),
                        plain(" text"),
                    ],
                }
            );
        }
    }

    mod parse_list_block_tests {
        use super::*;

        #[test]
        fn single_unordered_item() {
            assert_eq!(
                parse_list_block("- Item 1"),
                ListBlock {
                    ordered: false,
                    body: vec![ListItem {
                        body: vec![plain("Item 1")],
                    }],
                }
            );
        }
        #[test]
        fn multiple_unordered_items() {
            assert_eq!(
                parse_list_block("- Item 1\n- Item 2"),
                ListBlock {
                    ordered: false,
                    body: vec![
                        ListItem {
                            body: vec![plain("Item 1")],
                        },
                        ListItem {
                            body: vec![plain("Item 2")],
                        },
                    ],
                }
            );
        }
        #[test]
        fn single_ordered_item() {
            assert_eq!(
                parse_list_block("1. Item 1"),
                ListBlock {
                    ordered: true,
                    body: vec![ListItem {
                        body: vec![plain("Item 1")],
                    }],
                }
            );
        }
        #[test]
        fn multiple_ordered_items() {
            assert_eq!(
                parse_list_block("1. Item 1\n2. Item 2"),
                ListBlock {
                    ordered: true,
                    body: vec![
                        ListItem {
                            body: vec![plain("Item 1")],
                        },
                        ListItem {
                            body: vec![plain("Item 2")],
                        },
                    ],
                }
            );
        }
        #[test]
        fn list_with_styled_text() {
            assert_eq!(
                parse_list_block("- Item with **strong** and *italic* text"),
                ListBlock {
                    ordered: false,
                    body: vec![ListItem {
                        body: vec![
                            plain("Item with "),
                            tb(TextBodyStyle::Strong, "strong"),
                            plain(" and "),
                            tb(TextBodyStyle::Italic, "italic"),
                            plain(" text"),
                        ],
                    }],
                }
            );
        }
        #[test]
        fn list_with_link() {
            assert_eq!(
                parse_list_block("- Item with [link](https://example.com)\n- Item with [link2](https://example.com)"),
                ListBlock {
                    ordered: false,
                    body: vec![
                        ListItem {
                            body: vec![
                                plain("Item with "),
                                InlineContent::Link(Link {
                                    body: vec![TextBody {
                                        style: TextBodyStyle::Plain,
                                        value: "link".to_string(),
                                    }],
                                    url: "https://example.com".to_string(),
                                }),
                            ],
                        },
                        ListItem {
                            body: vec![
                                plain("Item with "),
                                InlineContent::Link(Link {
                                    body: vec![TextBody {
                                        style: TextBodyStyle::Plain,
                                        value: "link2".to_string(),
                                    }],
                                    url: "https://example.com".to_string(),
                                }),
                            ],
                        },
                    ],
                }
            );
        }
    }

    mod parse_thematic_break_tests {
        use super::*;

        #[test]
        fn three_hyphens() {
            assert_eq!(parse_blocks("---"), vec![Block::ThematicBreak]);
        }
        #[test]
        fn more_than_three_hyphens() {
            assert_eq!(parse_blocks("-----"), vec![Block::ThematicBreak]);
        }
        #[test]
        fn fewer_than_three_hyphens_is_paragraph() {
            assert_eq!(
                parse_blocks("--"),
                vec![Block::Paragraph(ParagraphBlock {
                    body: vec![plain("--")],
                })]
            );
        }
    }

    mod parse_code_block_tests {
        use super::*;

        #[test]
        fn basic_code_block_with_language() {
            assert_eq!(
                parse_code_block("```javascript\nconst x = 5;\nconsole.log(x);\n```"),
                CodeBlock {
                    lang: Some("javascript".to_string()),
                    body: "const x = 5;\nconsole.log(x);".to_string(),
                }
            );
        }
        #[test]
        fn code_block_without_language() {
            assert_eq!(
                parse_code_block("```\nfunction test() {\n  return true;\n}\n```"),
                CodeBlock {
                    lang: None,
                    body: "function test() {\n  return true;\n}".to_string(),
                }
            );
        }
        #[test]
        fn code_block_with_python() {
            assert_eq!(
                parse_code_block("```python\ndef hello():\n    print(\"Hello, World!\")\n```"),
                CodeBlock {
                    lang: Some("python".to_string()),
                    body: "def hello():\n    print(\"Hello, World!\")".to_string(),
                }
            );
        }
        #[test]
        fn code_block_with_special_characters() {
            assert_eq!(
                parse_code_block("```\n# Special chars: !@#$%^&*()\n```"),
                CodeBlock {
                    lang: None,
                    body: "# Special chars: !@#$%^&*()".to_string(),
                }
            );
        }
        #[test]
        fn code_block_with_empty_content() {
            assert_eq!(
                parse_code_block("```\n\n```"),
                CodeBlock {
                    lang: None,
                    body: "".to_string(),
                }
            );
        }
        #[test]
        fn code_block_with_indentation() {
            assert_eq!(
                parse_code_block(
                    "```typescript\nfunction example() {\n  const x = 1;\n  return x + 2;\n}\n```"
                ),
                CodeBlock {
                    lang: Some("typescript".to_string()),
                    body: "function example() {\n  const x = 1;\n  return x + 2;\n}".to_string(),
                }
            );
        }
    }

    mod parse_blocks_tests {
        use super::*;

        #[test]
        fn parses_heading_block() {
            assert_eq!(
                parse_blocks("# Heading 1"),
                vec![Block::Heading(HeadingBlock {
                    level: HeadingLevel::new(1).unwrap(),
                    body: vec![plain("Heading 1")],
                })]
            );
        }
        #[test]
        fn parses_quote_block() {
            assert_eq!(
                parse_blocks("> This is a quote"),
                vec![Block::Quote(QuoteBlock {
                    body: vec![plain("This is a quote")],
                })]
            );
        }
        #[test]
        fn parses_unordered_list_block() {
            assert_eq!(
                parse_blocks("- List item 1\n- List item 2"),
                vec![Block::List(ListBlock {
                    ordered: false,
                    body: vec![
                        ListItem {
                            body: vec![plain("List item 1")],
                        },
                        ListItem {
                            body: vec![plain("List item 2")],
                        },
                    ],
                })]
            );
        }
        #[test]
        fn parses_ordered_list_block() {
            assert_eq!(
                parse_blocks("1. List item 1\n2. List item 2"),
                vec![Block::List(ListBlock {
                    ordered: true,
                    body: vec![
                        ListItem {
                            body: vec![plain("List item 1")],
                        },
                        ListItem {
                            body: vec![plain("List item 2")],
                        },
                    ],
                })]
            );
        }
        #[test]
        fn parses_paragraph_by_default() {
            assert_eq!(
                parse_blocks("This is a paragraph"),
                vec![Block::Paragraph(ParagraphBlock {
                    body: vec![plain("This is a paragraph")],
                })]
            );
        }
        #[test]
        fn parses_paragraph_with_styled_text() {
            assert_eq!(
                parse_blocks("Paragraph with **strong** and *italic* text"),
                vec![Block::Paragraph(ParagraphBlock {
                    body: vec![
                        plain("Paragraph with "),
                        tb(TextBodyStyle::Strong, "strong"),
                        plain(" and "),
                        tb(TextBodyStyle::Italic, "italic"),
                        plain(" text"),
                    ],
                })]
            );
        }
        #[test]
        fn parses_image_block() {
            assert_eq!(
                parse_blocks("![Alt text](/path/to/image.jpg)(Image caption)"),
                vec![Block::Image(ImageBlock {
                    url: "/path/to/image.jpg".to_string(),
                    alt_text: "Alt text".to_string(),
                    caption: "Image caption".to_string(),
                })]
            );
        }
        #[test]
        fn parses_code_block() {
            assert_eq!(
                parse_blocks("```javascript\nconst x = 5;\nconsole.log(x);\n```"),
                vec![Block::Code(CodeBlock {
                    lang: Some("javascript".to_string()),
                    body: "const x = 5;\nconsole.log(x);".to_string(),
                })]
            );
        }
        #[test]
        fn parses_thematic_break() {
            assert_eq!(parse_blocks("---"), vec![Block::ThematicBreak]);
        }
    }
}
