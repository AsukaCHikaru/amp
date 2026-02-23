#[cfg(test)]
mod heading_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/heading.test.md").unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Heading test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about heading test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "heading-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.blocks.len(), 9);
        assert!(result.blocks.iter().all(|b| matches!(b, Block::Heading(_))));
    }

    #[test]
    fn should_parse_heading_levels_correctly() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let headings: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Heading(h) = b { Some(h) } else { None }
        }).collect();

        assert_eq!(headings[0].level, HeadingLevel::new(1).unwrap());
        assert_eq!(headings[1].level, HeadingLevel::new(2).unwrap());
        assert_eq!(headings[2].level, HeadingLevel::new(3).unwrap());
        assert_eq!(headings[3].level, HeadingLevel::new(4).unwrap());
        assert_eq!(headings[4].level, HeadingLevel::new(5).unwrap());
        assert_eq!(headings[5].level, HeadingLevel::new(6).unwrap());
        assert_eq!(headings[6].level, HeadingLevel::new(1).unwrap());
        assert_eq!(headings[7].level, HeadingLevel::new(3).unwrap());
        assert_eq!(headings[8].level, HeadingLevel::new(2).unwrap());
    }

    #[test]
    fn block_content() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        // H1 heading
        assert_eq!(result.blocks[0], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(1).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In sed purus vel nunc tempus posuere. Nulla nulla elit, convallis vitae vulputate vel, semper id justo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec vulputate tempor risus nec gravida. Pellentesque malesuada mauris tellus, eu tincidunt neque luctus eget. Vestibulum nisl est, gravida et condimentum et, dignissim quis sapien. Integer quis nunc id augue varius ultricies. Fusce eleifend felis tellus, interdum tristique sem egestas vitae. Donec bibendum massa quis dolor vehicula malesuada. Morbi porttitor sit amet neque vitae hendrerit. Aliquam sem felis, dictum ac dapibus vel, dictum vitae sapien.".to_string() })],
        }));

        // H2 heading
        assert_eq!(result.blocks[1], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(2).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec tellus ex. Mauris sapien lorem, accumsan ac efficitur in, luctus efficitur augue. In volutpat fermentum orci in porta. Aliquam mattis elementum nunc, volutpat laoreet nisi semper in. Proin cursus nisl elit. Proin at semper tellus. Aenean non commodo justo. Vivamus cursus imperdiet ipsum, eget sollicitudin nibh laoreet id. Curabitur pharetra dapibus enim quis viverra. Sed venenatis nibh nunc, vitae vulputate velit interdum eu. Sed sed lectus ex.".to_string() })],
        }));

        // H3 heading
        assert_eq!(result.blocks[2], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(3).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Integer quis nunc sodales, cursus mauris nec, dignissim purus. Nulla id mauris arcu. Phasellus et dui euismod, egestas nibh quis, molestie quam. Donec et metus volutpat, luctus massa ac, tempus velit. Vivamus quis consectetur odio. Ut egestas libero semper efficitur commodo. Integer fermentum odio a vulputate maximus. Nulla at maximus purus.".to_string() })],
        }));

        // H4 heading
        assert_eq!(result.blocks[3], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(4).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Donec nec arcu ligula. Morbi ultrices nibh quis turpis elementum, quis facilisis quam varius. Maecenas tempus ullamcorper lobortis. Sed varius tristique nibh. Pellentesque gravida id magna scelerisque ultricies. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin ullamcorper massa id nisl posuere maximus. Curabitur ornare hendrerit vulputate. Quisque ultricies eget quam et consectetur. Vestibulum scelerisque dignissim hendrerit. Donec neque ex, pulvinar vel eleifend eu, venenatis vel massa. Quisque vestibulum aliquet justo vitae sagittis. Pellentesque tellus odio, semper in consequat sagittis, bibendum ut mauris.".to_string() })],
        }));

        // H5 heading
        assert_eq!(result.blocks[4], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(5).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Quisque sit amet convallis urna. Etiam egestas laoreet eros at malesuada. Vivamus interdum molestie mi, sagittis consequat dolor pellentesque vitae. Quisque magna turpis, blandit sit amet risus at, lobortis cursus ante. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Proin nulla ipsum, vehicula sit amet lacus eget, posuere dapibus nunc. Aenean quis sagittis turpis, porta gravida magna. Phasellus eget ante vulputate, semper velit quis, faucibus tellus. Nunc id metus pellentesque, porta metus eget, malesuada ex. Sed mattis quam eget diam sodales, non pretium mauris semper. Nulla maximus porttitor enim maximus porta.".to_string() })],
        }));

        // H6 heading
        assert_eq!(result.blocks[5], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(6).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Vestibulum facilisis orci risus, at eleifend diam efficitur a. Proin sit amet ex id nisl ultrices dapibus. Phasellus elementum ipsum vitae ipsum suscipit, a consequat nisi ultricies. Suspendisse scelerisque justo eu pretium auctor. Nulla accumsan eros sit amet lorem convallis, eget tincidunt enim eleifend. Integer vel ultricies mauris, at dignissim nisl. Quisque consequat venenatis felis, a molestie lacus sollicitudin a. Curabitur ex elit, ultrices porta nisi at, porttitor lacinia turpis. Ut suscipit gravida mollis. Donec eu commodo eros, a bibendum orci. Phasellus in rutrum ante, ut tristique urna. Donec ultrices, quam at mollis tincidunt, neque quam interdum ipsum, vel dictum nisl elit pharetra nisi. Vestibulum finibus urna elit, a molestie nulla interdum posuere.".to_string() })],
        }));

        // Another H1 heading
        assert_eq!(result.blocks[6], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(1).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Fusce eu suscipit est, nec commodo magna. Phasellus vel imperdiet leo. Donec euismod tempus erat, et porttitor quam finibus eget. Pellentesque maximus fermentum nunc ut fermentum. Praesent eu convallis nunc. Duis mattis, sem nec rhoncus tincidunt, felis ante maximus orci, a luctus urna dui id est. Cras bibendum congue metus, sit amet vestibulum lectus laoreet vel. Aliquam fermentum fermentum eros, eget lacinia nulla imperdiet id.".to_string() })],
        }));

        // Another H3 heading
        assert_eq!(result.blocks[7], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(3).unwrap(),
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Sed in nulla vel diam consectetur imperdiet sit amet ut massa. Nam turpis nulla, fringilla in tortor id, laoreet sollicitudin tellus. Quisque rhoncus tincidunt dui sit amet semper. Donec lacinia, velit nec pretium pharetra, augue ante bibendum neque, quis scelerisque augue enim sed lectus. Quisque blandit dui sed aliquet varius. Aliquam convallis tortor et diam posuere, ac facilisis ante elementum. Quisque ac dolor nibh. Sed id felis nulla. Curabitur neque justo, eleifend et sodales eget, lacinia ut velit. Fusce sit amet justo sed nunc facilisis fermentum. In volutpat aliquet magna a finibus.".to_string() })],
        }));

        // H2 heading with styled text and links
        assert_eq!(result.blocks[8], Block::Heading(HeadingBlock {
            level: HeadingLevel::new(2).unwrap(),
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Sed felis metus, ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "sagittis".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " in ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "suscipit at,".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " consectetur ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "gravida".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " magna. ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "Orci varius natoque".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " penatibus ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "et".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " magnis dis ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "parturient montes, nascetur".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ridiculus mus. ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "Cras".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " odio enim, ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "congue vel erat vel,".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " maximus ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "gravida".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " tellus. ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "Phasellus facilisis mauris libero".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: ", sit amet hendrerit mi consequat vel. Suspendisse vulputate purus pharetra, posuere dolor non, dapibus magna. Maecenas et lacinia enim, sagittis convallis lacus. Vivamus imperdiet viverra mauris a vulputate. Suspendisse sollicitudin, augue blandit ultricies semper, turpis odio consectetur ipsum, non tincidunt erat massa id elit. Nam interdum blandit faucibus.".to_string() }),
            ],
        }));
    }
}

#[cfg(test)]
mod code_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/code.test.md").unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Code test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about code test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "code-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::Code(_))).collect();
        assert_eq!(code_blocks.len(), 5);
    }

    #[test]
    fn should_parse_single_line_code_with_language() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Code(c) = b { Some(c) } else { None }
        }).collect();

        assert_eq!(code_blocks[0].lang, Some("js".to_string()));
        assert_eq!(code_blocks[0].body, "// single line code");
    }

    #[test]
    fn should_parse_code_without_language() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Code(c) = b { Some(c) } else { None }
        }).collect();

        assert_eq!(code_blocks[1].lang, None);
        assert_eq!(code_blocks[1].body, "no lang");
    }

    #[test]
    fn should_parse_multi_line_code_with_language() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Code(c) = b { Some(c) } else { None }
        }).collect();

        assert_eq!(code_blocks[2].lang, Some("js".to_string()));
        assert_eq!(code_blocks[2].body, "// multiple lang code\n\nconst x = 5;\n\nconsole.log(x);");
    }

    #[test]
    fn should_parse_code_with_other_languages() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Code(c) = b { Some(c) } else { None }
        }).collect();

        assert_eq!(code_blocks[3].lang, Some("python".to_string()));
        assert_eq!(code_blocks[3].body, "# other lang");
    }

    #[test]
    fn should_parse_code_blocks_with_no_line_between() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let code_blocks: Vec<_> = result.blocks.iter().filter_map(|b| {
            if let Block::Code(c) = b { Some(c) } else { None }
        }).collect();

        assert_eq!(code_blocks[4].lang, Some("ts".to_string()));
        assert_eq!(code_blocks[4].body, "// no line between");
    }
}

#[cfg(test)]
mod image_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/image.test.md").unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Image test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about image test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "image-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let image_blocks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::Image(_))).collect();
        assert_eq!(image_blocks.len(), 7);
    }

    #[test]
    fn block_content() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.blocks[0], Block::Image(ImageBlock {
            url: "empty_alt_caption.jpeg".to_string(),
            alt_text: "".to_string(),
            caption: "".to_string(),
        }));

        assert_eq!(result.blocks[1], Block::Image(ImageBlock {
            url: "empty_caption.png".to_string(),
            alt_text: "alt".to_string(),
            caption: "".to_string(),
        }));

        assert_eq!(result.blocks[2], Block::Image(ImageBlock {
            url: "empty_alt.webp".to_string(),
            alt_text: "".to_string(),
            caption: "caption".to_string(),
        }));

        assert_eq!(result.blocks[3], Block::Image(ImageBlock {
            url: "full.gif".to_string(),
            alt_text: "alt".to_string(),
            caption: "caption".to_string(),
        }));

        assert_eq!(result.blocks[4], Block::Image(ImageBlock {
            url: "long_alt.png".to_string(),
            alt_text: "long alt".to_string(),
            caption: "".to_string(),
        }));

        assert_eq!(result.blocks[5], Block::Image(ImageBlock {
            url: "long_caption.png".to_string(),
            alt_text: "".to_string(),
            caption: "long caption".to_string(),
        }));

        assert_eq!(result.blocks[6], Block::Image(ImageBlock {
            url: "no_line_between.jpeg".to_string(),
            alt_text: "no line between".to_string(),
            caption: "".to_string(),
        }));
    }
}

#[cfg(test)]
mod list_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/list.test.md").unwrap()
    }

    fn get_list_blocks() -> Vec<ListBlock> {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());
        result.blocks.into_iter().filter_map(|b| {
            if let Block::List(l) = b { Some(l) } else { None }
        }).collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "List test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about list test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "list-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let lists = get_list_blocks();
        assert_eq!(lists.len(), 5);
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

        assert_eq!(lists[0].body[0], ListItem {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "single unordered".to_string() })],
        });
    }

    #[test]
    fn single_ordered_list_content() {
        let lists = get_list_blocks();

        assert_eq!(lists[1].body[0], ListItem {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "single ordered".to_string() })],
        });
    }

    #[test]
    fn multi_item_unordered_list_content() {
        let lists = get_list_blocks();

        for (i, expected) in ["unordered 1", "unordered 2", "unordered 3", "unordered 4", "unordered 5"].iter().enumerate() {
            assert_eq!(lists[2].body[i], ListItem {
                body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: expected.to_string() })],
            });
        }
    }

    #[test]
    fn multi_item_ordered_list_content() {
        let lists = get_list_blocks();

        for (i, expected) in ["ordered 1", "ordered 2", "ordered 3", "ordered 4", "ordered 5"].iter().enumerate() {
            assert_eq!(lists[3].body[i], ListItem {
                body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: expected.to_string() })],
            });
        }
    }

    #[test]
    fn styled_text_list_content() {
        let lists = get_list_blocks();

        // Plain text item
        assert_eq!(lists[4].body[0], ListItem {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string() })],
        });

        // Bold text item
        assert_eq!(lists[4].body[1], ListItem {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "Lorem".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ipsum dolor ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "sit amet, consectetur adipiscing elit.".to_string() }),
            ],
        });

        // Italic text item (underscore)
        assert_eq!(lists[4].body[2], ListItem {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "Lorem".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ipsum dolor ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "sit amet, consectetur adipiscing elit.".to_string() }),
            ],
        });

        // Italic text item (asterisk)
        assert_eq!(lists[4].body[3], ListItem {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "Lorem".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ipsum dolor ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "sit amet, consectetur adipiscing elit.".to_string() }),
            ],
        });

        // Code text item
        assert_eq!(lists[4].body[4], ListItem {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "Lorem".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ipsum dolor ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "sit amet, consectetur adipiscing elit.".to_string() }),
            ],
        });

        // Link item
        assert_eq!(lists[4].body[5], ListItem {
            body: vec![
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "Lorem".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ipsum dolor ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "sit amet, consectetur adipiscing elit.".to_string() }] }),
            ],
        });
    }
}

#[cfg(test)]
mod paragraph_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/paragraph.test.md").unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Paragraph test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about paragraph test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "paragraph-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.blocks.len(), 5);
        assert!(result.blocks.iter().all(|b| matches!(b, Block::Paragraph(_))));
    }

    #[test]
    fn block_content() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        // First paragraph
        assert_eq!(result.blocks[0], Block::Paragraph(ParagraphBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu metus egestas, eleifend tellus id, dignissim purus. Donec iaculis, dui ut pulvinar lacinia, massa magna fermentum elit, id faucibus augue orci non ex. Phasellus ultrices sem tellus, eu cursus mauris condimentum et. Morbi scelerisque sapien non erat venenatis, at volutpat sem consequat. Fusce id velit hendrerit, aliquet justo et, consectetur velit. Mauris tristique risus nunc, sit amet pulvinar felis venenatis non. Suspendisse eget ipsum fermentum, luctus est quis, lacinia lacus. Proin vulputate lectus quis porttitor tincidunt. Aenean eget ex ac justo hendrerit congue.".to_string() })],
        }));

        // Paragraph with line between
        assert_eq!(result.blocks[1], Block::Paragraph(ParagraphBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Paragraph with a line between. Cras porttitor eros nec cursus pharetra. Pellentesque ac blandit risus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Fusce nec elementum nisi. Donec efficitur lacus vel congue vehicula. Vestibulum eget sodales enim. Proin rutrum commodo erat ac lobortis. Aenean et egestas nulla, eu sodales nisl.".to_string() })],
        }));

        // Paragraph without line between
        assert_eq!(result.blocks[2], Block::Paragraph(ParagraphBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Paragraph without line between. Sed ut porttitor eros. Fusce gravida mi sed velit interdum, quis vehicula neque condimentum. Curabitur condimentum porttitor magna, sit amet dignissim ante ullamcorper in. In mattis velit sit amet orci rhoncus sodales. Vestibulum posuere accumsan cursus. Mauris nec libero tempor, tincidunt erat aliquam, rhoncus metus. Mauris cursus mattis elit, nec aliquam eros iaculis sed. Ut convallis dapibus faucibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi sit amet sodales sem, a eleifend turpis. Cras facilisis felis vitae nulla tempus facilisis. Aliquam erat volutpat. Nulla egestas sollicitudin nulla non blandit. Fusce ut sagittis nisl, quis malesuada nulla.".to_string() })],
        }));

        // Paragraph with many lines between
        assert_eq!(result.blocks[3], Block::Paragraph(ParagraphBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Paragraph with many lines between. Cras a iaculis velit. Cras volutpat dolor lorem, sit amet mattis odio pretium eget. Etiam id tellus nec nulla vulputate ornare vel quis neque. Suspendisse lorem dui, hendrerit in mollis porta, laoreet eget lectus. Curabitur et turpis faucibus, viverra dolor id, pharetra nisi. In ante dui, bibendum eu ex quis, dapibus facilisis tellus. Suspendisse vehicula, lorem sed vehicula condimentum, mi risus hendrerit leo, in ultricies odio tortor sed urna. Ut convallis, magna gravida bibendum ultrices, ex ex finibus leo, sit amet euismod est magna ut ex. Phasellus malesuada sapien sed sapien interdum interdum.".to_string() })],
        }));

        // Paragraph with styled texts
        assert_eq!(result.blocks[4], Block::Paragraph(ParagraphBlock {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Paragraph with styled texts. Vestibulum porta dapibus mi, ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "vitae".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " tincidunt ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "velit dignissim varius.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "Vivamus".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " tincidunt ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "in tortor porta feugiat.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "Suspendisse".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " vitae ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "velit sit amet odio accumsan eleifend efficitur vel eros.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "Praesent".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " sit ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "amet tincidunt mauris,".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " nec ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "dictum".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " augue. ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "Vivamus eget porttitor odio".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: ", id cursus nunc. Duis vel consequat mauris. Maecenas ut nunc a lorem convallis gravida in id est.".to_string() }),
            ],
        }));
    }
}

#[cfg(test)]
mod quote_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/quote.test.md").unwrap()
    }

    fn get_quote_blocks() -> Vec<QuoteBlock> {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());
        result.blocks.into_iter().filter_map(|b| {
            if let Block::Quote(q) = b { Some(q) } else { None }
        }).collect()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Quote test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about quote test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "quote-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let quotes = get_quote_blocks();
        assert_eq!(quotes.len(), 4);
    }

    #[test]
    fn single_line_quote_content() {
        let quotes = get_quote_blocks();

        assert_eq!(quotes[0], QuoteBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "single line quote".to_string() })],
        });
    }

    #[test]
    fn multiple_line_quote_content() {
        let quotes = get_quote_blocks();

        assert_eq!(quotes[1], QuoteBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "multiple line quote 1\nmultiple line quote 2\nmultiple line quote 3\nmultiple line quote 4\nmultiple line quote 5".to_string() })],
        });
    }

    #[test]
    fn styled_text_quote_content() {
        let quotes = get_quote_blocks();

        assert_eq!(quotes[2], QuoteBlock {
            body: vec![
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "Lorem ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "ipsum".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " dolor sit ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Strong, value: "amet, consectetur adipiscing elit.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "\nLorem ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "ipsum".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " dolor sit ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "amet, consectetur adipiscing elit.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "\nLorem ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "ipsum".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " dolor sit ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Italic, value: "amet, consectetur adipiscing elit.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "\nLorem ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "ipsum".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " dolor sit ".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Code, value: "amet, consectetur adipiscing elit.".to_string() }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "\nLorem ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "ipsum".to_string() }] }),
                InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: " dolor ".to_string() }),
                InlineContent::Link(Link { url: "https://example.com".to_string(), body: vec![TextBody { style: TextBodyStyle::Plain, value: "sit amet, consectetur adipiscing elit.".to_string() }] }),
            ],
        });
    }

    #[test]
    fn empty_line_in_quote() {
        let quotes = get_quote_blocks();

        assert_eq!(quotes[3], QuoteBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "empty line in quote 1\n\nempty line in quote 2\n\nempty line in quote 3".to_string() })],
        });
    }
}

#[cfg(test)]
mod thematic_break_tests {
    use crate::parser::amp::Amp;
    use crate::types::*;

    fn get_markdown() -> String {
        std::fs::read_to_string("test/thematicBreak.test.md").unwrap()
    }

    #[test]
    fn should_parse_frontmatter() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.frontmatter.get("title").unwrap(), "Thematic break test");
        assert_eq!(result.frontmatter.get("description").unwrap(), "Sentence of the description, which is about thematic break test. This property exist for the purpose of, no other than frontmatter testing.");
        assert_eq!(result.frontmatter.get("date").unwrap(), "2020-01-01");
        assert_eq!(result.frontmatter.get("datetime").unwrap(), "2020-01-01 12:00");
        assert_eq!(result.frontmatter.get("pathname").unwrap(), "thematic-break-test");
        assert_eq!(result.frontmatter.get("category").unwrap(), "test");
    }

    #[test]
    fn should_parse_blocks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let breaks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::ThematicBreak)).collect();
        assert_eq!(breaks.len(), 4);
    }

    #[test]
    fn should_parse_standard_thematic_break() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let breaks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::ThematicBreak)).collect();
        assert_eq!(breaks[0], &Block::ThematicBreak);
    }

    #[test]
    fn should_parse_thematic_break_with_more_hyphens() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let breaks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::ThematicBreak)).collect();
        assert_eq!(breaks[1], &Block::ThematicBreak);
    }

    #[test]
    fn should_parse_consecutive_thematic_breaks() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        let breaks: Vec<_> = result.blocks.iter().filter(|b| matches!(b, Block::ThematicBreak)).collect();
        assert_eq!(breaks[2], &Block::ThematicBreak);
        assert_eq!(breaks[3], &Block::ThematicBreak);
    }

    #[test]
    fn should_not_parse_invalid_thematic_break() {
        let amp = Amp::new();
        let result = amp.parse(&get_markdown());

        assert_eq!(result.blocks[1], Block::Paragraph(ParagraphBlock {
            body: vec![InlineContent::TextBody(TextBody { style: TextBodyStyle::Plain, value: "--".to_string() })],
        }));
    }
}
