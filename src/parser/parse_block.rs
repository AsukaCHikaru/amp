use std::sync::LazyLock;

use regex::Regex;

use crate::types::{Block, CodeBlock, HeadingBlock, ImageBlock, ListBlock, QuoteBlock};

static PARAGRAPH_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([\s\S]+?)(?:\n|$)").expect("Invalid regex"));

fn parse_blocks(input: &str) -> Vec<Block> {
    if input.trim().is_empty() {
        return vec![];
    }

    if let Some(caps) = PARAGRAPH_PATTERN.captures(input) {}

    vec![]
}

fn parse_heading_block(input: &str) -> HeadingBlock {
    todo!()
}
fn parse_image_block(input: &str) -> ImageBlock {
    todo!()
}
fn parse_code_block(input: &str) -> CodeBlock {
    todo!()
}
fn parse_list_block(input: &str) -> ListBlock {
    todo!()
}
fn parse_quote_block(input: &str) -> QuoteBlock {
    todo!()
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
                Block::Image(ImageBlock {
                    url: "/images/sample.jpg".to_string(),
                    alt_text: "Sample Alt Text".to_string(),
                    caption: "This is a caption".to_string(),
                })
            );
        }
        #[test]
        fn image_with_spaces_in_url() {
            assert_eq!(
                parse_image_block("![Alt text](/path/to/image with spaces.jpg)(Caption)"),
                Block::Image(ImageBlock {
                    url: "/path/to/image with spaces.jpg".to_string(),
                    alt_text: "Alt text".to_string(),
                    caption: "Caption".to_string(),
                })
            );
        }
        #[test]
        fn image_with_empty_alt_text() {
            assert_eq!(
                parse_image_block("![](/images/no-alt.jpg)(Image without alt text)"),
                Block::Image(ImageBlock {
                    url: "/images/no-alt.jpg".to_string(),
                    alt_text: "".to_string(),
                    caption: "Image without alt text".to_string(),
                })
            );
        }
        #[test]
        fn image_without_caption() {
            assert_eq!(
                parse_image_block("![Alt text only](/images/no-caption.jpg)"),
                Block::Image(ImageBlock {
                    url: "/images/no-caption.jpg".to_string(),
                    alt_text: "Alt text only".to_string(),
                    caption: "".to_string(),
                })
            );
        }
    }

    mod parse_heading_block_tests {
        use super::*;

        #[test]
        fn parses_h1() {
            assert_eq!(
                parse_heading_block("# Heading 1"),
                Block::Heading(HeadingBlock {
                    level: HeadingLevel::new(1).unwrap(),
                    body: vec![plain("Heading 1")],
                })
            );
        }
        #[test]
        fn parses_h2() {
            assert_eq!(
                parse_heading_block("## Heading 2"),
                Block::Heading(HeadingBlock {
                    level: HeadingLevel::new(2).unwrap(),
                    body: vec![plain("Heading 2")],
                })
            );
        }
        #[test]
        fn parses_h6() {
            assert_eq!(
                parse_heading_block("###### Heading 6"),
                Block::Heading(HeadingBlock {
                    level: HeadingLevel::new(6).unwrap(),
                    body: vec![plain("Heading 6")],
                })
            );
        }
        #[test]
        fn heading_with_styled_text() {
            assert_eq!(
                parse_heading_block("# Heading with **strong** and *italic* text"),
                Block::Heading(HeadingBlock {
                    level: HeadingLevel::new(1).unwrap(),
                    body: vec![
                        plain("Heading with "),
                        tb(TextBodyStyle::Strong, "strong"),
                        plain(" and "),
                        tb(TextBodyStyle::Italic, "italic"),
                        plain(" text"),
                    ],
                })
            );
        }
    }

    mod parse_quote_block_tests {
        use super::*;

        #[test]
        fn simple_quote() {
            assert_eq!(
                parse_quote_block("> This is a quote"),
                Block::Quote(QuoteBlock {
                    body: vec![plain("This is a quote")],
                })
            );
        }
        #[test]
        fn quote_with_styled_text() {
            assert_eq!(
                parse_quote_block("> Quote with **strong** and *italic* text"),
                Block::Quote(QuoteBlock {
                    body: vec![
                        plain("Quote with "),
                        tb(TextBodyStyle::Strong, "strong"),
                        plain(" and "),
                        tb(TextBodyStyle::Italic, "italic"),
                        plain(" text"),
                    ],
                })
            );
        }
    }

    mod parse_list_block_tests {
        use super::*;

        #[test]
        fn single_unordered_item() {
            assert_eq!(
                parse_list_block("- Item 1"),
                Block::List(ListBlock {
                    ordered: false,
                    body: vec![ListItem {
                        body: vec![plain("Item 1")],
                    }],
                })
            );
        }
        #[test]
        fn multiple_unordered_items() {
            assert_eq!(
                parse_list_block("- Item 1\n- Item 2"),
                Block::List(ListBlock {
                    ordered: false,
                    body: vec![
                        ListItem {
                            body: vec![plain("Item 1")],
                        },
                        ListItem {
                            body: vec![plain("Item 2")],
                        },
                    ],
                })
            );
        }
        #[test]
        fn single_ordered_item() {
            assert_eq!(
                parse_list_block("1. Item 1"),
                Block::List(ListBlock {
                    ordered: true,
                    body: vec![ListItem {
                        body: vec![plain("Item 1")],
                    }],
                })
            );
        }
        #[test]
        fn multiple_ordered_items() {
            assert_eq!(
                parse_list_block("1. Item 1\n2. Item 2"),
                Block::List(ListBlock {
                    ordered: true,
                    body: vec![
                        ListItem {
                            body: vec![plain("Item 1")],
                        },
                        ListItem {
                            body: vec![plain("Item 2")],
                        },
                    ],
                })
            );
        }
        #[test]
        fn list_with_styled_text() {
            assert_eq!(
                parse_list_block("- Item with **strong** and *italic* text"),
                Block::List(ListBlock {
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
                })
            );
        }
        #[test]
        fn list_with_link() {
            assert_eq!(
                parse_list_block("- Item with [link](https://example.com)\n- Item with [link2](https://example.com)"),
                Block::List(ListBlock {
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
                })
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
                Block::Code(CodeBlock {
                    lang: Some("javascript".to_string()),
                    body: "const x = 5;\nconsole.log(x);".to_string(),
                })
            );
        }
        #[test]
        fn code_block_without_language() {
            assert_eq!(
                parse_code_block("```\nfunction test() {\n  return true;\n}\n```"),
                Block::Code(CodeBlock {
                    lang: None,
                    body: "function test() {\n  return true;\n}".to_string(),
                })
            );
        }
        #[test]
        fn code_block_with_python() {
            assert_eq!(
                parse_code_block("```python\ndef hello():\n    print(\"Hello, World!\")\n```"),
                Block::Code(CodeBlock {
                    lang: Some("python".to_string()),
                    body: "def hello():\n    print(\"Hello, World!\")".to_string(),
                })
            );
        }
        #[test]
        fn code_block_with_special_characters() {
            assert_eq!(
                parse_code_block("```\n# Special chars: !@#$%^&*()\n```"),
                Block::Code(CodeBlock {
                    lang: None,
                    body: "# Special chars: !@#$%^&*()".to_string(),
                })
            );
        }
        #[test]
        fn code_block_with_empty_content() {
            assert_eq!(
                parse_code_block("```\n\n```"),
                Block::Code(CodeBlock {
                    lang: None,
                    body: "".to_string(),
                })
            );
        }
        #[test]
        fn code_block_with_indentation() {
            assert_eq!(
                parse_code_block(
                    "```typescript\nfunction example() {\n  const x = 1;\n  return x + 2;\n}\n```"
                ),
                Block::Code(CodeBlock {
                    lang: Some("typescript".to_string()),
                    body: "function example() {\n  const x = 1;\n  return x + 2;\n}".to_string(),
                })
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
