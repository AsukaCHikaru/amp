use wasm_bindgen::prelude::*;

use crate::{
    parser::{
        parse_block::parse_blocks,
        parse_frontmatter::parse_frontmatter,
        split::{split, SplitResult},
    },
    types::ParseResult,
};

#[wasm_bindgen]
pub struct Amp {}

#[wasm_bindgen]
impl Amp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Amp {}
    }

    #[wasm_bindgen]
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

    fn plain(value: &str) -> InlineContent {
        InlineContent::TextBody(TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        })
    }

    fn tb(style: TextBodyStyle, value: &str) -> InlineContent {
        InlineContent::TextBody(TextBody {
            style,
            value: value.to_string(),
        })
    }

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
            .filter_map(|b| match b {
                Block::Heading(h) => Some(h),
                _ => None,
            })
            .collect();
        assert!(headings.len() >= 5);
        assert_eq!(headings[0].level, HeadingLevel::new(1).unwrap());
        assert_eq!(headings[0].body[0], plain("Main Heading"));
        assert_eq!(headings[1].level, HeadingLevel::new(2).unwrap());
        assert_eq!(headings[1].body[0], plain("Features Section"));

        // Verify quote
        let quotes: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Quote(q) => Some(q),
                _ => None,
            })
            .collect();
        assert_eq!(quotes.len(), 1);
        assert_eq!(
            quotes[0].body[0],
            plain(
                "This is a quote block with some important information.\nIt spans multiple lines."
            )
        );

        // Verify lists
        let lists: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::List(l) => Some(l),
                _ => None,
            })
            .collect();
        assert_eq!(lists.len(), 2);
        assert_eq!(lists[0].ordered, false);
        assert_eq!(lists[0].body[0].body[0], plain("First item"));
        assert_eq!(lists[0].body[1].body[0], plain("Second item"));
        assert_eq!(lists[0].body[2].body[0], plain("Third item"));
        assert_eq!(lists[1].ordered, true);
        assert_eq!(lists[1].body[0].body[0], plain("Numbered item one"));
        assert_eq!(lists[1].body[1].body[0], plain("Numbered item two"));
        assert_eq!(lists[1].body[2].body[0], plain("Numbered item three"));

        // Verify code
        let codes: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Code(c) => Some(c),
                _ => None,
            })
            .collect();
        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].lang, Some("javascript".to_string()));
        assert_eq!(codes[0].body, "const x = 5;\nconsole.log(x);");

        // Verify image
        let images: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Image(i) => Some(i),
                _ => None,
            })
            .collect();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].url, "image.jpg");
        assert_eq!(images[0].alt_text, "Alt text");
        assert_eq!(images[0].caption, "This is a caption");

        // Verify thematic break
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak(_)))
            .collect();
        assert_eq!(breaks.len(), 1);

        // Verify paragraph with styled text
        let paragraphs: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(p),
                _ => None,
            })
            .collect();
        assert!(paragraphs.len() >= 2);
        assert_eq!(
            paragraphs[0].body,
            vec![
                plain("This is an introductory paragraph with "),
                tb(TextBodyStyle::Strong, "bold"),
                plain(" and "),
                tb(TextBodyStyle::Italic, "italic"),
                plain(" text."),
            ]
        );
    }
}
