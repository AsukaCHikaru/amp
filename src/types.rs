use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "lowercase")]
pub enum TextBodyStyle {
    Plain,
    Italic,
    Strong,
    Code,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "textBody")]
pub struct TextBody {
    pub style: TextBodyStyle,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "link")]
pub struct Link {
    pub body: Vec<TextBody>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(untagged)]
pub enum InlineContent {
    TextBody(TextBody),
    Link(Link),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "paragraph")]
pub struct ParagraphBlock {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct HeadingLevel(u8);
impl HeadingLevel {
    pub fn new(level: u8) -> Result<HeadingLevel, String> {
        match level {
            1..=6 => Ok(HeadingLevel(level)),
            _ => Err("Unexpected level".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "heading")]
pub struct HeadingBlock {
    pub body: Vec<InlineContent>,
    pub level: HeadingLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "quote")]
pub struct QuoteBlock {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "listItem")]
pub struct ListItem {
    pub body: Vec<InlineContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "list")]
pub struct ListBlock {
    pub items: Vec<ListItem>,
    pub ordered: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "image", rename_all = "camelCase")]
pub struct ImageBlock {
    pub url: String,
    pub alt_text: String,
    pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "code")]
pub struct CodeBlock {
    pub lang: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", rename = "thematicBreak")]
pub struct ThematicBreak {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(untagged)]
pub enum Block {
    Paragraph(ParagraphBlock),
    Heading(HeadingBlock),
    Quote(QuoteBlock),
    List(ListBlock),
    Image(ImageBlock),
    Code(CodeBlock),
    ThematicBreak(ThematicBreak),
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, hashmap_as_object)]
pub struct ParseResult {
    pub frontmatter: HashMap<String, String>,
    pub blocks: Vec<Block>,
}
