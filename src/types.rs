use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextBodyStyle {
    Plain,
    Italic,
    Strong,
    Code,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextBody {
    pub style: TextBodyStyle,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub body: Vec<TextBody>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InlineContent {
    TextBody(TextBody),
    Link(Link),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParagraphBlock {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadingLevel(u8);
impl HeadingLevel {
    pub fn new(level: u8) -> Result<HeadingLevel, String> {
        match level {
            1..=6 => Ok(HeadingLevel(level)),
            _ => Err("Unexpected level".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadingBlock {
    pub body: Vec<InlineContent>,
    pub level: HeadingLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteBlock {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListBlock {
    pub body: Vec<ListItem>,
    pub ordered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageBlock {
    pub url: String,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub lang: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Block {
    Paragraph(ParagraphBlock),
    Heading(HeadingBlock),
    Quote(QuoteBlock),
    List(ListBlock),
    Image(ImageBlock),
    Code(CodeBlock),
    ThematicBreak,
}

#[derive(Serialize, Deserialize)]
pub struct ParseResult {
    pub frontmatter: HashMap<String, String>,
    pub blocks: Vec<Block>,
}
