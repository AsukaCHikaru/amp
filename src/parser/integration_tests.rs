#[cfg(test)]
mod heading_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/heading.test.md").unwrap();
        amp.parse(&md)
    }

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

    fn link(url: &str, body: Vec<TextBody>) -> InlineContent {
        InlineContent::Link(Link {
            url: url.to_string(),
            body,
        })
    }

    fn plain_tb(value: &str) -> TextBody {
        TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        }
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "Heading test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about heading test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "heading-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let headings: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Heading(h) => Some(h),
                _ => None,
            })
            .collect();
        assert_eq!(headings.len(), 9);
        assert_eq!(result.blocks.len(), 9);
    }

    #[test]
    fn should_parse_heading_levels_correctly() {
        let result = parse_markdown();
        let headings: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Heading(h) => Some(h),
                _ => None,
            })
            .collect();
        let expected_levels: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 1, 3, 2];
        for (i, level) in expected_levels.iter().enumerate() {
            assert_eq!(headings[i].level, HeadingLevel::new(*level).unwrap());
        }
    }

    #[test]
    fn block_content() {
        let result = parse_markdown();
        let headings: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Heading(h) => Some(h),
                _ => None,
            })
            .collect();

        // H1
        assert_eq!(headings[0].body[0], plain("Lorem ipsum dolor sit amet, consectetur adipiscing elit. In sed purus vel nunc tempus posuere. Nulla nulla elit, convallis vitae vulputate vel, semper id justo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec vulputate tempor risus nec gravida. Pellentesque malesuada mauris tellus, eu tincidunt neque luctus eget. Vestibulum nisl est, gravida et condimentum et, dignissim quis sapien. Integer quis nunc id augue varius ultricies. Fusce eleifend felis tellus, interdum tristique sem egestas vitae. Donec bibendum massa quis dolor vehicula malesuada. Morbi porttitor sit amet neque vitae hendrerit. Aliquam sem felis, dictum ac dapibus vel, dictum vitae sapien."));

        // H2 heading with styled text and links (block[8])
        assert_eq!(headings[8].level, HeadingLevel::new(2).unwrap());
        let body = &headings[8].body;
        assert_eq!(body[0], plain("Sed felis metus, "));
        assert_eq!(body[1], tb(TextBodyStyle::Italic, "sagittis"));
        assert_eq!(
            body[13],
            link("https://example.com", vec![plain_tb("Cras")])
        );
        assert_eq!(body[17], tb(TextBodyStyle::Code, "gravida"));
    }
}

#[cfg(test)]
mod code_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/code.test.md").unwrap();
        amp.parse(&md)
    }

    fn get_code_blocks() -> Vec<CodeBlock> {
        let result = parse_markdown();
        result
            .blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::Code(c) => Some(c),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "Code test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about code test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "code-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_code_blocks().len(), 5);
    }

    #[test]
    fn should_parse_single_line_code_with_language() {
        let codes = get_code_blocks();
        assert_eq!(codes[0].lang, Some("js".to_string()));
        assert_eq!(codes[0].body, "// single line code");
    }

    #[test]
    fn should_parse_code_without_language() {
        let codes = get_code_blocks();
        assert_eq!(codes[1].lang, None);
        assert_eq!(codes[1].body, "no lang");
    }

    #[test]
    fn should_parse_multi_line_code_with_language() {
        let codes = get_code_blocks();
        assert_eq!(codes[2].lang, Some("js".to_string()));
        assert_eq!(
            codes[2].body,
            "// multiple lang code\n\nconst x = 5;\n\nconsole.log(x);"
        );
    }

    #[test]
    fn should_parse_code_with_other_languages() {
        let codes = get_code_blocks();
        assert_eq!(codes[3].lang, Some("python".to_string()));
        assert_eq!(codes[3].body, "# other lang");
    }

    #[test]
    fn should_parse_code_blocks_with_no_line_between() {
        let codes = get_code_blocks();
        assert_eq!(codes[4].lang, Some("ts".to_string()));
        assert_eq!(codes[4].body, "// no line between");
    }
}

#[cfg(test)]
mod image_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/image.test.md").unwrap();
        amp.parse(&md)
    }

    fn get_image_blocks() -> Vec<ImageBlock> {
        let result = parse_markdown();
        result
            .blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::Image(i) => Some(i),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "Image test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about image test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "image-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_image_blocks().len(), 7);
    }

    #[test]
    fn block_content() {
        let images = get_image_blocks();

        assert_eq!(
            images[0],
            ImageBlock {
                url: "empty_alt_caption.jpeg".to_string(),
                alt_text: "".to_string(),
                caption: "".to_string()
            }
        );
        assert_eq!(
            images[1],
            ImageBlock {
                url: "empty_caption.png".to_string(),
                alt_text: "alt".to_string(),
                caption: "".to_string()
            }
        );
        assert_eq!(
            images[2],
            ImageBlock {
                url: "empty_alt.webp".to_string(),
                alt_text: "".to_string(),
                caption: "caption".to_string()
            }
        );
        assert_eq!(
            images[3],
            ImageBlock {
                url: "full.gif".to_string(),
                alt_text: "alt".to_string(),
                caption: "caption".to_string()
            }
        );
        assert_eq!(
            images[4],
            ImageBlock {
                url: "long_alt.png".to_string(),
                alt_text: "long alt".to_string(),
                caption: "".to_string()
            }
        );
        assert_eq!(
            images[5],
            ImageBlock {
                url: "long_caption.png".to_string(),
                alt_text: "".to_string(),
                caption: "long caption".to_string()
            }
        );
        assert_eq!(
            images[6],
            ImageBlock {
                url: "no_line_between.jpeg".to_string(),
                alt_text: "no line between".to_string(),
                caption: "".to_string()
            }
        );
    }
}

#[cfg(test)]
mod list_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/list.test.md").unwrap();
        amp.parse(&md)
    }

    fn get_list_blocks() -> Vec<ListBlock> {
        let result = parse_markdown();
        result
            .blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::List(l) => Some(l),
                _ => None,
            })
            .collect()
    }

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

    fn link(url: &str, body: Vec<TextBody>) -> InlineContent {
        InlineContent::Link(Link {
            url: url.to_string(),
            body,
        })
    }

    fn plain_tb(value: &str) -> TextBody {
        TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        }
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "List test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about list test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "list-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_list_blocks().len(), 5);
    }

    #[test]
    fn should_identify_ordered_and_unordered() {
        let lists = get_list_blocks();
        assert_eq!(lists[0].ordered, false);
        assert_eq!(lists[1].ordered, true);
        assert_eq!(lists[2].ordered, false);
        assert_eq!(lists[3].ordered, true);
        assert_eq!(lists[4].ordered, false);
    }

    #[test]
    fn should_parse_list_items_correctly() {
        let lists = get_list_blocks();
        assert_eq!(lists[0].body.len(), 1);
        assert_eq!(lists[1].body.len(), 1);
        assert_eq!(lists[2].body.len(), 5);
        assert_eq!(lists[3].body.len(), 5);
        assert_eq!(lists[4].body.len(), 6);
    }

    #[test]
    fn single_unordered_list_content() {
        let lists = get_list_blocks();
        assert_eq!(lists[0].body[0].body, vec![plain("single unordered")]);
    }

    #[test]
    fn single_ordered_list_content() {
        let lists = get_list_blocks();
        assert_eq!(lists[1].body[0].body, vec![plain("single ordered")]);
    }

    #[test]
    fn multi_item_unordered_list_content() {
        let lists = get_list_blocks();
        for (i, expected) in [
            "unordered 1",
            "unordered 2",
            "unordered 3",
            "unordered 4",
            "unordered 5",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(lists[2].body[i].body, vec![plain(expected)]);
        }
    }

    #[test]
    fn multi_item_ordered_list_content() {
        let lists = get_list_blocks();
        for (i, expected) in [
            "ordered 1",
            "ordered 2",
            "ordered 3",
            "ordered 4",
            "ordered 5",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(lists[3].body[i].body, vec![plain(expected)]);
        }
    }

    #[test]
    fn styled_text_list_content() {
        let lists = get_list_blocks();
        let styled = &lists[4].body;

        // Plain
        assert_eq!(
            styled[0].body,
            vec![plain(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )]
        );

        // Bold
        assert_eq!(
            styled[1].body,
            vec![
                tb(TextBodyStyle::Strong, "Lorem"),
                plain(" ipsum dolor "),
                tb(
                    TextBodyStyle::Strong,
                    "sit amet, consectetur adipiscing elit."
                ),
            ]
        );

        // Italic (underscore)
        assert_eq!(
            styled[2].body,
            vec![
                tb(TextBodyStyle::Italic, "Lorem"),
                plain(" ipsum dolor "),
                tb(
                    TextBodyStyle::Italic,
                    "sit amet, consectetur adipiscing elit."
                ),
            ]
        );

        // Italic (asterisk)
        assert_eq!(
            styled[3].body,
            vec![
                tb(TextBodyStyle::Italic, "Lorem"),
                plain(" ipsum dolor "),
                tb(
                    TextBodyStyle::Italic,
                    "sit amet, consectetur adipiscing elit."
                ),
            ]
        );

        // Code
        assert_eq!(
            styled[4].body,
            vec![
                tb(TextBodyStyle::Code, "Lorem"),
                plain(" ipsum dolor "),
                tb(
                    TextBodyStyle::Code,
                    "sit amet, consectetur adipiscing elit."
                ),
            ]
        );

        // Links
        assert_eq!(
            styled[5].body,
            vec![
                link("https://example.com", vec![plain_tb("Lorem")]),
                plain(" ipsum dolor "),
                link(
                    "https://example.com",
                    vec![plain_tb("sit amet, consectetur adipiscing elit.")]
                ),
            ]
        );
    }
}

#[cfg(test)]
mod paragraph_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/paragraph.test.md").unwrap();
        amp.parse(&md)
    }

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

    fn link(url: &str, body: Vec<TextBody>) -> InlineContent {
        InlineContent::Link(Link {
            url: url.to_string(),
            body,
        })
    }

    fn plain_tb(value: &str) -> TextBody {
        TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        }
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "Paragraph test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about paragraph test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(
            result.frontmatter.get("pathname").unwrap(),
            "paragraph-test"
        );
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let paragraphs: Vec<_> = result
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(p),
                _ => None,
            })
            .collect();
        assert_eq!(paragraphs.len(), 5);
        assert_eq!(result.blocks.len(), 5);
    }

    #[test]
    fn block_content() {
        let result = parse_markdown();
        let paragraphs: Vec<_> = result
            .blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(p),
                _ => None,
            })
            .collect();

        // First paragraph
        assert_eq!(paragraphs[0].body[0], plain("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu metus egestas, eleifend tellus id, dignissim purus. Donec iaculis, dui ut pulvinar lacinia, massa magna fermentum elit, id faucibus augue orci non ex. Phasellus ultrices sem tellus, eu cursus mauris condimentum et. Morbi scelerisque sapien non erat venenatis, at volutpat sem consequat. Fusce id velit hendrerit, aliquet justo et, consectetur velit. Mauris tristique risus nunc, sit amet pulvinar felis venenatis non. Suspendisse eget ipsum fermentum, luctus est quis, lacinia lacus. Proin vulputate lectus quis porttitor tincidunt. Aenean eget ex ac justo hendrerit congue."));

        // Paragraph with line between
        assert_eq!(paragraphs[1].body[0], plain("Paragraph with a line between. Cras porttitor eros nec cursus pharetra. Pellentesque ac blandit risus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Fusce nec elementum nisi. Donec efficitur lacus vel congue vehicula. Vestibulum eget sodales enim. Proin rutrum commodo erat ac lobortis. Aenean et egestas nulla, eu sodales nisl."));

        // Paragraph without line between
        assert_eq!(paragraphs[2].body[0], plain("Paragraph without line between. Sed ut porttitor eros. Fusce gravida mi sed velit interdum, quis vehicula neque condimentum. Curabitur condimentum porttitor magna, sit amet dignissim ante ullamcorper in. In mattis velit sit amet orci rhoncus sodales. Vestibulum posuere accumsan cursus. Mauris nec libero tempor, tincidunt erat aliquam, rhoncus metus. Mauris cursus mattis elit, nec aliquam eros iaculis sed. Ut convallis dapibus faucibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi sit amet sodales sem, a eleifend turpis. Cras facilisis felis vitae nulla tempus facilisis. Aliquam erat volutpat. Nulla egestas sollicitudin nulla non blandit. Fusce ut sagittis nisl, quis malesuada nulla."));

        // Paragraph with many lines between
        assert_eq!(paragraphs[3].body[0], plain("Paragraph with many lines between. Cras a iaculis velit. Cras volutpat dolor lorem, sit amet mattis odio pretium eget. Etiam id tellus nec nulla vulputate ornare vel quis neque. Suspendisse lorem dui, hendrerit in mollis porta, laoreet eget lectus. Curabitur et turpis faucibus, viverra dolor id, pharetra nisi. In ante dui, bibendum eu ex quis, dapibus facilisis tellus. Suspendisse vehicula, lorem sed vehicula condimentum, mi risus hendrerit leo, in ultricies odio tortor sed urna. Ut convallis, magna gravida bibendum ultrices, ex ex finibus leo, sit amet euismod est magna ut ex. Phasellus malesuada sapien sed sapien interdum interdum."));

        // Paragraph with styled texts
        assert_eq!(paragraphs[4].body, vec![
            plain("Paragraph with styled texts. Vestibulum porta dapibus mi, "),
            tb(TextBodyStyle::Strong, "vitae"),
            plain(" tincidunt "),
            tb(TextBodyStyle::Strong, "velit dignissim varius."),
            plain(" "),
            tb(TextBodyStyle::Italic, "Vivamus"),
            plain(" tincidunt "),
            tb(TextBodyStyle::Italic, "in tortor porta feugiat."),
            plain(" "),
            tb(TextBodyStyle::Italic, "Suspendisse"),
            plain(" vitae "),
            tb(TextBodyStyle::Italic, "velit sit amet odio accumsan eleifend efficitur vel eros."),
            plain(" "),
            link("https://example.com", vec![plain_tb("Praesent")]),
            plain(" sit "),
            link("https://example.com", vec![plain_tb("amet tincidunt mauris,")]),
            plain(" nec "),
            tb(TextBodyStyle::Code, "dictum"),
            plain(" augue. "),
            tb(TextBodyStyle::Code, "Vivamus eget porttitor odio"),
            plain(", id cursus nunc. Duis vel consequat mauris. Maecenas ut nunc a lorem convallis gravida in id est."),
        ]);
    }
}

#[cfg(test)]
mod quote_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/quote.test.md").unwrap();
        amp.parse(&md)
    }

    fn get_quote_blocks() -> Vec<QuoteBlock> {
        let result = parse_markdown();
        result
            .blocks
            .into_iter()
            .filter_map(|b| match b {
                Block::Quote(q) => Some(q),
                _ => None,
            })
            .collect()
    }

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

    fn link(url: &str, body: Vec<TextBody>) -> InlineContent {
        InlineContent::Link(Link {
            url: url.to_string(),
            body,
        })
    }

    fn plain_tb(value: &str) -> TextBody {
        TextBody {
            style: TextBodyStyle::Plain,
            value: value.to_string(),
        }
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result.frontmatter.get("title").unwrap(), "Quote test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about quote test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "quote-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_quote_blocks().len(), 4);
    }

    #[test]
    fn single_line_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(quotes[0].body, vec![plain("single line quote")]);
    }

    #[test]
    fn multiple_line_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(quotes[1].body, vec![plain("multiple line quote 1\nmultiple line quote 2\nmultiple line quote 3\nmultiple line quote 4\nmultiple line quote 5")]);
    }

    #[test]
    fn styled_text_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[2].body,
            vec![
                plain("Lorem "),
                tb(TextBodyStyle::Strong, "ipsum"),
                plain(" dolor sit "),
                tb(TextBodyStyle::Strong, "amet, consectetur adipiscing elit."),
                plain("\nLorem "),
                tb(TextBodyStyle::Italic, "ipsum"),
                plain(" dolor sit "),
                tb(TextBodyStyle::Italic, "amet, consectetur adipiscing elit."),
                plain("\nLorem "),
                tb(TextBodyStyle::Italic, "ipsum"),
                plain(" dolor sit "),
                tb(TextBodyStyle::Italic, "amet, consectetur adipiscing elit."),
                plain("\nLorem "),
                tb(TextBodyStyle::Code, "ipsum"),
                plain(" dolor sit "),
                tb(TextBodyStyle::Code, "amet, consectetur adipiscing elit."),
                plain("\nLorem "),
                link("https://example.com", vec![plain_tb("ipsum")]),
                plain(" dolor "),
                link(
                    "https://example.com",
                    vec![plain_tb("sit amet, consectetur adipiscing elit.")]
                ),
            ]
        );
    }

    #[test]
    fn empty_line_in_quote() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[3].body,
            vec![plain(
                "empty line in quote 1\n\nempty line in quote 2\n\nempty line in quote 3"
            )]
        );
    }
}

#[cfg(test)]
mod thematic_break_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn parse_markdown() -> ParseResult {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/thematicBreak.test.md").unwrap();
        amp.parse(&md)
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(
            result.frontmatter.get("title").unwrap(),
            "Thematic break test"
        );
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about thematic break test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(
            result.frontmatter.get("datetime").unwrap(),
            "2020-01-01 12:00"
        );
        assert_eq!(
            result.frontmatter.get("pathname").unwrap(),
            "thematic-break-test"
        );
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak(_)))
            .collect();
        assert_eq!(breaks.len(), 4);
    }

    #[test]
    fn should_parse_standard_thematic_break() {
        let result = parse_markdown();
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak(_)))
            .collect();
        assert_eq!(*breaks[0], Block::ThematicBreak(ThematicBreak {}));
    }

    #[test]
    fn should_parse_thematic_break_with_more_hyphens() {
        let result = parse_markdown();
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak(_)))
            .collect();
        assert_eq!(*breaks[1], Block::ThematicBreak(ThematicBreak {}));
    }

    #[test]
    fn should_parse_consecutive_thematic_breaks() {
        let result = parse_markdown();
        let breaks: Vec<_> = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::ThematicBreak(_)))
            .collect();
        assert_eq!(*breaks[2], Block::ThematicBreak(ThematicBreak {}));
        assert_eq!(*breaks[3], Block::ThematicBreak(ThematicBreak {}));
    }

    #[test]
    fn should_not_parse_invalid_thematic_break() {
        let result = parse_markdown();
        assert_eq!(
            result.blocks[1],
            Block::Paragraph(ParagraphBlock {
                body: vec![InlineContent::TextBody(TextBody {
                    style: TextBodyStyle::Plain,
                    value: "--".to_string(),
                })]
            })
        );
    }
}
