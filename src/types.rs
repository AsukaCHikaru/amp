use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextBodyStyle {
  Plain,
  Italic,
  Strong,
  Code,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBody{
  pub style: TextBodyStyle,
  pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
  pub body: Vec<TextBody>,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InlineContent {
  TextBody(TextBody),
  Link(Link)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphBlock {
  body: Vec<InlineContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingLevel(u8);
impl HeadingLevel {
  pub fn new(level: u8) -> Result<HeadingLevel, String> {
    match level {
        1..=6 => Ok(HeadingLevel(level)),
        _ => Err("Unexpected level".to_string()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingBlock {
  body: Vec<InlineContent>,
  level: HeadingLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteBlock {
  body: Vec<InlineContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
  body: Vec<InlineContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBlock {
  body: Vec<ListItem>,
  ordered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageBlock {
  url: String,
  alt_text: String,
  caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
  lang: Option<String>,
  body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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