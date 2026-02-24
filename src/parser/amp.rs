use napi_derive::napi;

use crate::{
    parser::{
        parse_block::parse_blocks,
        parse_frontmatter::parse_frontmatter,
        split::{split, SplitResult},
    },
    types::ParseResult,
};

#[napi]
pub struct Amp {}

impl Amp {
    fn internal_parse(input: &str) -> ParseResult {
        let SplitResult { head, body } = split(input);
        let frontmatter = parse_frontmatter(&head);
        let blocks = parse_blocks(&body);
        ParseResult {
            frontmatter,
            blocks,
        }
    }
}

#[napi]
impl Amp {
    #[napi(constructor)]
    pub fn new() -> Self {
        Amp {}
    }

    #[napi]
    pub fn parse(&self, input: String) -> String {
        let result = Self::internal_parse(&input);
        serde_json::to_string(&result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

        let result: serde_json::Value = serde_json::from_str(&amp.parse(input.to_string())).unwrap();

        // Verify frontmatter
        assert_eq!(result["frontmatter"]["title"], "Test Document");
        assert_eq!(result["frontmatter"]["author"], "Test Author");

        // Verify block count
        let blocks = result["blocks"].as_array().unwrap();
        assert!(blocks.len() > 10);

        // Verify headings
        let headings: Vec<_> = blocks.iter().filter(|b| b["type"] == "heading").collect();
        assert!(headings.len() >= 5);
        assert_eq!(headings[0]["level"], 1);
        assert_eq!(headings[0]["body"][0], json!({"type": "textBody", "style": "plain", "value": "Main Heading"}));
        assert_eq!(headings[1]["level"], 2);
        assert_eq!(headings[1]["body"][0], json!({"type": "textBody", "style": "plain", "value": "Features Section"}));

        // Verify quote
        let quotes: Vec<_> = blocks.iter().filter(|b| b["type"] == "quote").collect();
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0]["body"][0], json!({"type": "textBody", "style": "plain", "value": "This is a quote block with some important information.\nIt spans multiple lines."}));

        // Verify lists
        let lists: Vec<_> = blocks.iter().filter(|b| b["type"] == "list").collect();
        assert_eq!(lists.len(), 2);
        assert_eq!(lists[0]["ordered"], false);
        assert_eq!(lists[0]["body"][0]["body"][0]["value"], "First item");
        assert_eq!(lists[0]["body"][1]["body"][0]["value"], "Second item");
        assert_eq!(lists[0]["body"][2]["body"][0]["value"], "Third item");
        assert_eq!(lists[1]["ordered"], true);
        assert_eq!(lists[1]["body"][0]["body"][0]["value"], "Numbered item one");
        assert_eq!(lists[1]["body"][1]["body"][0]["value"], "Numbered item two");
        assert_eq!(lists[1]["body"][2]["body"][0]["value"], "Numbered item three");

        // Verify code
        let codes: Vec<_> = blocks.iter().filter(|b| b["type"] == "code").collect();
        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0]["lang"], "javascript");
        assert_eq!(codes[0]["body"], "const x = 5;\nconsole.log(x);");

        // Verify image
        let images: Vec<_> = blocks.iter().filter(|b| b["type"] == "image").collect();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0], &json!({"type": "image", "url": "image.jpg", "altText": "Alt text", "caption": "This is a caption"}));

        // Verify thematic break
        let breaks: Vec<_> = blocks.iter().filter(|b| b["type"] == "thematicBreak").collect();
        assert_eq!(breaks.len(), 1);

        // Verify paragraph with styled text
        let paragraphs: Vec<_> = blocks.iter().filter(|b| b["type"] == "paragraph").collect();
        assert!(paragraphs.len() >= 2);
        assert_eq!(paragraphs[0]["body"], json!([
            {"type": "textBody", "style": "plain", "value": "This is an introductory paragraph with "},
            {"type": "textBody", "style": "strong", "value": "bold"},
            {"type": "textBody", "style": "plain", "value": " and "},
            {"type": "textBody", "style": "italic", "value": "italic"},
            {"type": "textBody", "style": "plain", "value": " text."}
        ]));
    }
}
