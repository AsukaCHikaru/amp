use crate::{
    parser::{
        parse_block::parse_blocks,
        parse_frontmatter::parse_frontmatter,
        split::{split, SplitResult},
    },
    types::ParseResult,
};

pub struct Amp {}
impl Amp {
    pub fn new() -> Self {
        Amp {}
    }
    pub fn parse(&self, input: &str) -> ParseResult {
        let SplitResult { head, body } = split(input);
        let frontmatter = parse_frontmatter(&head);
        let blocks = parse_blocks(&body);
        ParseResult {
            frontmatter,
            blocks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn instantiate() {
        let _amp = Amp::new();
    }

    #[test]
    fn parse_complex_document() {
        let amp = Amp::new();
        let input = r#"---
title: Test Document
author: Test Author
---

# Main Heading

This is an introductory paragraph with **bold** and *italic* text.

## Features Section

> This is a quote block with some important information.
> It spans multiple lines.

### List of Items

- First item
- Second item
- Third item

1. Numbered item one
2. Numbered item two
3. Numbered item three

#### Code Example

```javascript
const x = 5;
console.log(x);
```

![Alt text](image.jpg)(This is a caption)

---

##### Final Notes

This is the final paragraph before we end."#;

        let result = amp.parse(input);

        // Verify frontmatter
        assert_eq!(result.frontmatter.get("title").unwrap(), "Test Document");
        assert_eq!(result.frontmatter.get("author").unwrap(), "Test Author");

        // Verify block count
        assert!(result.blocks.len() > 10);

        // Verify headings
        let headings: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Heading(_)))
            .collect();
        assert!(headings.len() >= 5);
        assert_eq!(
            headings[0],
            &Block::Heading(HeadingBlock {
                level: HeadingLevel::new(1).unwrap(),
                body: vec![InlineContent::TextBody(TextBody {
                    style: TextBodyStyle::Plain,
                    value: "Main Heading".to_string()
                })],
            })
        );
        assert_eq!(
            headings[1],
            &Block::Heading(HeadingBlock {
                level: HeadingLevel::new(2).unwrap(),
                body: vec![InlineContent::TextBody(TextBody {
                    style: TextBodyStyle::Plain,
                    value: "Features Section".to_string()
                })],
            })
        );

        // Verify quote
        let quotes: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Quote(_)))
            .collect();
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0], &Block::Quote(QuoteBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "This is a quote block with some important information.\nIt spans multiple lines.".to_string() })],
        }));

        // Verify lists
        let lists: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::List(_)))
            .collect();
        assert_eq!(lists.len(), 2);
        assert_eq!(
            lists[0],
            &Block::List(ListBlock {
                ordered: false,
                body: vec![
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "First item".to_string()
                        })]
                    },
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "Second item".to_string()
                        })]
                    },
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "Third item".to_string()
                        })]
                    },
                ],
            })
        );
        assert_eq!(
            lists[1],
            &Block::List(ListBlock {
                ordered: true,
                body: vec![
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "Numbered item one".to_string()
                        })]
                    },
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "Numbered item two".to_string()
                        })]
                    },
                    ListItem {
                        body: vec![InlineContent::TextBody(TextBody {
                            style: TextBodyStyle::Plain,
                            value: "Numbered item three".to_string()
                        })]
                    },
                ],
            })
        );

        // Verify code
        let codes: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Code(_)))
            .collect();
        assert_eq!(codes.len(), 1);
        assert_eq!(
            codes[0],
            &Block::Code(CodeBlock {
                lang: Some("javascript".to_string()),
                body: "const x = 5;\nconsole.log(x);".to_string()
            })
        );

        // Verify image
        let images: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Image(_)))
            .collect();
        assert_eq!(images.len(), 1);
        assert_eq!(
            images[0],
            &Block::Image(ImageBlock {
                url: "image.jpg".to_string(),
                alt_text: "Alt text".to_string(),
                caption: "This is a caption".to_string()
            })
        );

        // Verify thematic break
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak))
            .collect();
        assert_eq!(breaks.len(), 1);

        // Verify paragraph with styled text
        let paragraphs: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Paragraph(_)))
            .collect();
        assert!(paragraphs.len() >= 2);
        assert_eq!(
            paragraphs[0],
            &Block::Paragraph(ParagraphBlock {
                body: vec![
                    InlineContent::TextBody(TextBody {
                        style: TextBodyStyle::Plain,
                        value: "This is an introductory paragraph with ".to_string()
                    }),
                    InlineContent::TextBody(TextBody {
                        style: TextBodyStyle::Strong,
                        value: "bold".to_string()
                    }),
                    InlineContent::TextBody(TextBody {
                        style: TextBodyStyle::Plain,
                        value: " and ".to_string()
                    }),
                    InlineContent::TextBody(TextBody {
                        style: TextBodyStyle::Italic,
                        value: "italic".to_string()
                    }),
                    InlineContent::TextBody(TextBody {
                        style: TextBodyStyle::Plain,
                        value: " text.".to_string()
                    }),
                ],
            })
        );
    }
}
