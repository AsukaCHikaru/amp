#[cfg(test)]
mod heading_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/heading.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Heading test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about heading test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "heading-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();
        assert_eq!(blocks.len(), 9);
        assert!(blocks.iter().all(|b| b["type"] == "heading"));
    }

    #[test]
    fn should_parse_heading_levels_correctly() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();
        let expected_levels = [1, 2, 3, 4, 5, 6, 1, 3, 2];
        for (i, level) in expected_levels.iter().enumerate() {
            assert_eq!(blocks[i]["level"], *level);
        }
    }

    #[test]
    fn block_content() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();

        // H1
        assert_eq!(blocks[0]["body"][0]["style"], "plain");
        assert_eq!(blocks[0]["body"][0]["value"], "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In sed purus vel nunc tempus posuere. Nulla nulla elit, convallis vitae vulputate vel, semper id justo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec vulputate tempor risus nec gravida. Pellentesque malesuada mauris tellus, eu tincidunt neque luctus eget. Vestibulum nisl est, gravida et condimentum et, dignissim quis sapien. Integer quis nunc id augue varius ultricies. Fusce eleifend felis tellus, interdum tristique sem egestas vitae. Donec bibendum massa quis dolor vehicula malesuada. Morbi porttitor sit amet neque vitae hendrerit. Aliquam sem felis, dictum ac dapibus vel, dictum vitae sapien.");

        // H2 heading with styled text and links (block[8])
        assert_eq!(blocks[8]["level"], 2);
        let body = blocks[8]["body"].as_array().unwrap();
        assert_eq!(
            body[0],
            json!({"type": "textBody", "style": "plain", "value": "Sed felis metus, "})
        );
        assert_eq!(
            body[1],
            json!({"type": "textBody", "style": "italic", "value": "sagittis"})
        );
        assert_eq!(
            body[13],
            json!({"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "Cras"}]})
        );
        assert_eq!(
            body[17],
            json!({"type": "textBody", "style": "code", "value": "gravida"})
        );
    }
}

#[cfg(test)]
mod code_tests {
    use crate::parser::amp::Amp;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/code.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    fn get_code_blocks() -> Vec<serde_json::Value> {
        let result = parse_markdown();
        result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "code")
            .cloned()
            .collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Code test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about code test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "code-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_code_blocks().len(), 5);
    }

    #[test]
    fn should_parse_single_line_code_with_language() {
        let codes = get_code_blocks();
        assert_eq!(codes[0]["lang"], "js");
        assert_eq!(codes[0]["body"], "// single line code");
    }

    #[test]
    fn should_parse_code_without_language() {
        let codes = get_code_blocks();
        assert!(codes[1]["lang"].is_null());
        assert_eq!(codes[1]["body"], "no lang");
    }

    #[test]
    fn should_parse_multi_line_code_with_language() {
        let codes = get_code_blocks();
        assert_eq!(codes[2]["lang"], "js");
        assert_eq!(
            codes[2]["body"],
            "// multiple lang code\n\nconst x = 5;\n\nconsole.log(x);"
        );
    }

    #[test]
    fn should_parse_code_with_other_languages() {
        let codes = get_code_blocks();
        assert_eq!(codes[3]["lang"], "python");
        assert_eq!(codes[3]["body"], "# other lang");
    }

    #[test]
    fn should_parse_code_blocks_with_no_line_between() {
        let codes = get_code_blocks();
        assert_eq!(codes[4]["lang"], "ts");
        assert_eq!(codes[4]["body"], "// no line between");
    }
}

#[cfg(test)]
mod image_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/image.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Image test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about image test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "image-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let images: Vec<_> = result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "image")
            .collect();
        assert_eq!(images.len(), 7);
    }

    #[test]
    fn block_content() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();

        assert_eq!(
            blocks[0],
            json!({"type": "image", "url": "empty_alt_caption.jpeg", "altText": "", "caption": ""})
        );
        assert_eq!(
            blocks[1],
            json!({"type": "image", "url": "empty_caption.png", "altText": "alt", "caption": ""})
        );
        assert_eq!(
            blocks[2],
            json!({"type": "image", "url": "empty_alt.webp", "altText": "", "caption": "caption"})
        );
        assert_eq!(
            blocks[3],
            json!({"type": "image", "url": "full.gif", "altText": "alt", "caption": "caption"})
        );
        assert_eq!(
            blocks[4],
            json!({"type": "image", "url": "long_alt.png", "altText": "long alt", "caption": ""})
        );
        assert_eq!(
            blocks[5],
            json!({"type": "image", "url": "long_caption.png", "altText": "", "caption": "long caption"})
        );
        assert_eq!(
            blocks[6],
            json!({"type": "image", "url": "no_line_between.jpeg", "altText": "no line between", "caption": ""})
        );
    }
}

#[cfg(test)]
mod list_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/list.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    fn get_list_blocks() -> Vec<serde_json::Value> {
        let result = parse_markdown();
        result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "list")
            .cloned()
            .collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "List test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about list test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "list-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_list_blocks().len(), 5);
    }

    #[test]
    fn should_identify_ordered_and_unordered() {
        let lists = get_list_blocks();
        assert_eq!(lists[0]["ordered"], false);
        assert_eq!(lists[1]["ordered"], true);
        assert_eq!(lists[2]["ordered"], false);
        assert_eq!(lists[3]["ordered"], true);
        assert_eq!(lists[4]["ordered"], false);
    }

    #[test]
    fn should_parse_list_items_correctly() {
        let lists = get_list_blocks();
        assert_eq!(lists[0]["body"].as_array().unwrap().len(), 1);
        assert_eq!(lists[1]["body"].as_array().unwrap().len(), 1);
        assert_eq!(lists[2]["body"].as_array().unwrap().len(), 5);
        assert_eq!(lists[3]["body"].as_array().unwrap().len(), 5);
        assert_eq!(lists[4]["body"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn single_unordered_list_content() {
        let lists = get_list_blocks();
        assert_eq!(
            lists[0]["body"][0]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "single unordered"}])
        );
    }

    #[test]
    fn single_ordered_list_content() {
        let lists = get_list_blocks();
        assert_eq!(
            lists[1]["body"][0]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "single ordered"}])
        );
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
            assert_eq!(
                lists[2]["body"][i]["body"],
                json!([{"type": "textBody", "style": "plain", "value": expected}])
            );
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
            assert_eq!(
                lists[3]["body"][i]["body"],
                json!([{"type": "textBody", "style": "plain", "value": expected}])
            );
        }
    }

    #[test]
    fn styled_text_list_content() {
        let lists = get_list_blocks();
        let styled = &lists[4]["body"];

        // Plain
        assert_eq!(
            styled[0]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "Lorem ipsum dolor sit amet, consectetur adipiscing elit."}])
        );

        // Bold
        assert_eq!(
            styled[1]["body"],
            json!([
                {"type": "textBody", "style": "strong", "value": "Lorem"},
                {"type": "textBody", "style": "plain", "value": " ipsum dolor "},
                {"type": "textBody", "style": "strong", "value": "sit amet, consectetur adipiscing elit."}
            ])
        );

        // Italic (underscore)
        assert_eq!(
            styled[2]["body"],
            json!([
                {"type": "textBody", "style": "italic", "value": "Lorem"},
                {"type": "textBody", "style": "plain", "value": " ipsum dolor "},
                {"type": "textBody", "style": "italic", "value": "sit amet, consectetur adipiscing elit."}
            ])
        );

        // Italic (asterisk)
        assert_eq!(
            styled[3]["body"],
            json!([
                {"type": "textBody", "style": "italic", "value": "Lorem"},
                {"type": "textBody", "style": "plain", "value": " ipsum dolor "},
                {"type": "textBody", "style": "italic", "value": "sit amet, consectetur adipiscing elit."}
            ])
        );

        // Code
        assert_eq!(
            styled[4]["body"],
            json!([
                {"type": "textBody", "style": "code", "value": "Lorem"},
                {"type": "textBody", "style": "plain", "value": " ipsum dolor "},
                {"type": "textBody", "style": "code", "value": "sit amet, consectetur adipiscing elit."}
            ])
        );

        // Links
        assert_eq!(
            styled[5]["body"],
            json!([
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "Lorem"}]},
                {"type": "textBody", "style": "plain", "value": " ipsum dolor "},
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "sit amet, consectetur adipiscing elit."}]}
            ])
        );
    }
}

#[cfg(test)]
mod paragraph_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/paragraph.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Paragraph test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about paragraph test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "paragraph-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();
        assert_eq!(blocks.len(), 5);
        assert!(blocks.iter().all(|b| b["type"] == "paragraph"));
    }

    #[test]
    fn block_content() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();

        // First paragraph
        assert_eq!(blocks[0]["body"][0]["value"], "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu metus egestas, eleifend tellus id, dignissim purus. Donec iaculis, dui ut pulvinar lacinia, massa magna fermentum elit, id faucibus augue orci non ex. Phasellus ultrices sem tellus, eu cursus mauris condimentum et. Morbi scelerisque sapien non erat venenatis, at volutpat sem consequat. Fusce id velit hendrerit, aliquet justo et, consectetur velit. Mauris tristique risus nunc, sit amet pulvinar felis venenatis non. Suspendisse eget ipsum fermentum, luctus est quis, lacinia lacus. Proin vulputate lectus quis porttitor tincidunt. Aenean eget ex ac justo hendrerit congue.");

        // Paragraph with line between
        assert_eq!(blocks[1]["body"][0]["value"], "Paragraph with a line between. Cras porttitor eros nec cursus pharetra. Pellentesque ac blandit risus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Fusce nec elementum nisi. Donec efficitur lacus vel congue vehicula. Vestibulum eget sodales enim. Proin rutrum commodo erat ac lobortis. Aenean et egestas nulla, eu sodales nisl.");

        // Paragraph without line between
        assert_eq!(blocks[2]["body"][0]["value"], "Paragraph without line between. Sed ut porttitor eros. Fusce gravida mi sed velit interdum, quis vehicula neque condimentum. Curabitur condimentum porttitor magna, sit amet dignissim ante ullamcorper in. In mattis velit sit amet orci rhoncus sodales. Vestibulum posuere accumsan cursus. Mauris nec libero tempor, tincidunt erat aliquam, rhoncus metus. Mauris cursus mattis elit, nec aliquam eros iaculis sed. Ut convallis dapibus faucibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi sit amet sodales sem, a eleifend turpis. Cras facilisis felis vitae nulla tempus facilisis. Aliquam erat volutpat. Nulla egestas sollicitudin nulla non blandit. Fusce ut sagittis nisl, quis malesuada nulla.");

        // Paragraph with many lines between
        assert_eq!(blocks[3]["body"][0]["value"], "Paragraph with many lines between. Cras a iaculis velit. Cras volutpat dolor lorem, sit amet mattis odio pretium eget. Etiam id tellus nec nulla vulputate ornare vel quis neque. Suspendisse lorem dui, hendrerit in mollis porta, laoreet eget lectus. Curabitur et turpis faucibus, viverra dolor id, pharetra nisi. In ante dui, bibendum eu ex quis, dapibus facilisis tellus. Suspendisse vehicula, lorem sed vehicula condimentum, mi risus hendrerit leo, in ultricies odio tortor sed urna. Ut convallis, magna gravida bibendum ultrices, ex ex finibus leo, sit amet euismod est magna ut ex. Phasellus malesuada sapien sed sapien interdum interdum.");

        // Paragraph with styled texts
        assert_eq!(
            blocks[4]["body"],
            json!([
                {"type": "textBody", "style": "plain", "value": "Paragraph with styled texts. Vestibulum porta dapibus mi, "},
                {"type": "textBody", "style": "strong", "value": "vitae"},
                {"type": "textBody", "style": "plain", "value": " tincidunt "},
                {"type": "textBody", "style": "strong", "value": "velit dignissim varius."},
                {"type": "textBody", "style": "plain", "value": " "},
                {"type": "textBody", "style": "italic", "value": "Vivamus"},
                {"type": "textBody", "style": "plain", "value": " tincidunt "},
                {"type": "textBody", "style": "italic", "value": "in tortor porta feugiat."},
                {"type": "textBody", "style": "plain", "value": " "},
                {"type": "textBody", "style": "italic", "value": "Suspendisse"},
                {"type": "textBody", "style": "plain", "value": " vitae "},
                {"type": "textBody", "style": "italic", "value": "velit sit amet odio accumsan eleifend efficitur vel eros."},
                {"type": "textBody", "style": "plain", "value": " "},
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "Praesent"}]},
                {"type": "textBody", "style": "plain", "value": " sit "},
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "amet tincidunt mauris,"}]},
                {"type": "textBody", "style": "plain", "value": " nec "},
                {"type": "textBody", "style": "code", "value": "dictum"},
                {"type": "textBody", "style": "plain", "value": " augue. "},
                {"type": "textBody", "style": "code", "value": "Vivamus eget porttitor odio"},
                {"type": "textBody", "style": "plain", "value": ", id cursus nunc. Duis vel consequat mauris. Maecenas ut nunc a lorem convallis gravida in id est."}
            ])
        );
    }
}

#[cfg(test)]
mod quote_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/quote.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    fn get_quote_blocks() -> Vec<serde_json::Value> {
        let result = parse_markdown();
        result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "quote")
            .cloned()
            .collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Quote test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about quote test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "quote-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        assert_eq!(get_quote_blocks().len(), 4);
    }

    #[test]
    fn single_line_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[0]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "single line quote"}])
        );
    }

    #[test]
    fn multiple_line_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[1]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "multiple line quote 1\nmultiple line quote 2\nmultiple line quote 3\nmultiple line quote 4\nmultiple line quote 5"}])
        );
    }

    #[test]
    fn styled_text_quote_content() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[2]["body"],
            json!([
                {"type": "textBody", "style": "plain", "value": "Lorem "},
                {"type": "textBody", "style": "strong", "value": "ipsum"},
                {"type": "textBody", "style": "plain", "value": " dolor sit "},
                {"type": "textBody", "style": "strong", "value": "amet, consectetur adipiscing elit."},
                {"type": "textBody", "style": "plain", "value": "\nLorem "},
                {"type": "textBody", "style": "italic", "value": "ipsum"},
                {"type": "textBody", "style": "plain", "value": " dolor sit "},
                {"type": "textBody", "style": "italic", "value": "amet, consectetur adipiscing elit."},
                {"type": "textBody", "style": "plain", "value": "\nLorem "},
                {"type": "textBody", "style": "italic", "value": "ipsum"},
                {"type": "textBody", "style": "plain", "value": " dolor sit "},
                {"type": "textBody", "style": "italic", "value": "amet, consectetur adipiscing elit."},
                {"type": "textBody", "style": "plain", "value": "\nLorem "},
                {"type": "textBody", "style": "code", "value": "ipsum"},
                {"type": "textBody", "style": "plain", "value": " dolor sit "},
                {"type": "textBody", "style": "code", "value": "amet, consectetur adipiscing elit."},
                {"type": "textBody", "style": "plain", "value": "\nLorem "},
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "ipsum"}]},
                {"type": "textBody", "style": "plain", "value": " dolor "},
                {"type": "link", "url": "https://example.com", "body": [{"style": "plain", "value": "sit amet, consectetur adipiscing elit."}]}
            ])
        );
    }

    #[test]
    fn empty_line_in_quote() {
        let quotes = get_quote_blocks();
        assert_eq!(
            quotes[3]["body"],
            json!([{"type": "textBody", "style": "plain", "value": "empty line in quote 1\n\nempty line in quote 2\n\nempty line in quote 3"}])
        );
    }
}

#[cfg(test)]
mod thematic_break_tests {
    use crate::parser::amp::Amp;
    use serde_json::json;

    fn parse_markdown() -> serde_json::Value {
        let amp = Amp::new();
        let md = std::fs::read_to_string("test/thematicBreak.test.md").unwrap();
        serde_json::from_str(&amp.parse(md)).unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let result = parse_markdown();
        assert_eq!(result["frontmatter"]["title"], "Thematic break test");
        assert_eq!(result["frontmatter"]["description"], "Sentence of the description, which is about thematic break test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result["frontmatter"]["date"], "2020-01-01");
        assert_eq!(result["frontmatter"]["datetime"], "2020-01-01 12:00");
        assert_eq!(result["frontmatter"]["pathname"], "thematic-break-test");
        assert_eq!(result["frontmatter"]["category"], "test");
    }

    #[test]
    fn should_parse_blocks() {
        let result = parse_markdown();
        let breaks: Vec<_> = result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "thematicBreak")
            .collect();
        assert_eq!(breaks.len(), 4);
    }

    #[test]
    fn should_parse_standard_thematic_break() {
        let result = parse_markdown();
        let breaks: Vec<_> = result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "thematicBreak")
            .collect();
        assert_eq!(breaks[0], &json!({"type": "thematicBreak"}));
    }

    #[test]
    fn should_parse_thematic_break_with_more_hyphens() {
        let result = parse_markdown();
        let breaks: Vec<_> = result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "thematicBreak")
            .collect();
        assert_eq!(breaks[1], &json!({"type": "thematicBreak"}));
    }

    #[test]
    fn should_parse_consecutive_thematic_breaks() {
        let result = parse_markdown();
        let breaks: Vec<_> = result["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["type"] == "thematicBreak")
            .collect();
        assert_eq!(breaks[2], &json!({"type": "thematicBreak"}));
        assert_eq!(breaks[3], &json!({"type": "thematicBreak"}));
    }

    #[test]
    fn should_not_parse_invalid_thematic_break() {
        let result = parse_markdown();
        let blocks = result["blocks"].as_array().unwrap();
        assert_eq!(
            blocks[1],
            json!({"type": "paragraph", "body": [{"type": "textBody", "style": "plain", "value": "--"}]})
        );
    }
}
