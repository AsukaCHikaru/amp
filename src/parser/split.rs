use std::sync::LazyLock;

use regex::Regex;

static SPLIT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(---\n[\s\S]*?---)\n*").expect("Invalid regex"));

struct SplitResult {
    head: String,
    body: String,
}

fn split(input: &str) -> SplitResult {
    let trimmed = input.trim();

    match SPLIT_PATTERN.captures(trimmed) {
        Some(captured) => {
            let head = captured.get(1).unwrap().as_str().to_string();
            SplitResult {
                body: trimmed[captured.get(0).unwrap().end()..].trim().to_string(),
                head,
            }
        }
        None => SplitResult {
            head: "".to_string(),
            body: input.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_markdown_with_head_and_body() {
        let input = r#"---
title: Test Title
tags: tag1, tag2
---
This is the body content.

With multiple paragraphs."#;

        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
title: Test Title
tags: tag1, tag2
---"#
        );
        assert_eq!(
            body,
            r#"This is the body content.

With multiple paragraphs."#
        )
    }

    #[test]
    fn handle_text_with_only_frontmatter() {
        let input = r#"---
title: Just Frontmatter
author: Test Author
---"#;

        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
title: Just Frontmatter
author: Test Author
---"#
        );
        assert_eq!(body, "");
    }

    #[test]
    fn handle_text_with_only_body() {
        let input = r#"This is just body content.

No frontmatter here."#;

        let SplitResult { head, body } = split(input);
        assert_eq!(head, "");
        assert_eq!(
            body,
            r#"This is just body content.

No frontmatter here."#
        );
    }

    #[test]
    fn handle_empty_string() {
        let input = "";
        let SplitResult { head, body } = split(input);
        assert_eq!(head, "");
        assert_eq!(body, "");
    }

    #[test]
    fn handle_frontmatter_with_special_characters() {
        let input = r#"---
title: Special * Characters & Symbols
description: This has some "quotes" and 'apostrophes'
---
Body content here."#;
        let SplitResult { head, body } = split(input);

        assert_eq!(
            head,
            r#"---
title: Special * Characters & Symbols
description: This has some "quotes" and 'apostrophes'
---"#
        );
        assert_eq!(body, "Body content here.");
    }

    #[test]
    fn handle_frontmatter_with_numbers() {
        let input = r#"---
count: 42
rating: 4.5
---
Numeric frontmatter test."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
count: 42
rating: 4.5
---"#
        );
        assert_eq!(body, "Numeric frontmatter test.");
    }

    #[test]
    fn handle_frontmatter_with_array() {
        let input = r#"---
tags: one, two, three
categories: cat1, cat2
---
Array frontmatter test."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
tags: one, two, three
categories: cat1, cat2
---"#
        );
        assert_eq!(body, "Array frontmatter test.");
    }

    #[test]
    fn handle_malformed_frontmatter() {
        let input = r#"---
This is not properly formatted
frontmatter without colons
---
Body content."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
This is not properly formatted
frontmatter without colons
---"#
        );
        assert_eq!(body, "Body content.");
    }

    #[test]
    fn handle_broken_frontmatter() {
        let input = r#"---
This is not properly formatted
frontmatter without colons
--
Body content."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(head, "");
        assert_eq!(
            body,
            r#"---
This is not properly formatted
frontmatter without colons
--
Body content."#
        );
    }

    #[test]
    fn handle_frontmatter_followed_by_thematic_break() {
        let input = r#"---
title: Document with Thematic Break
---
Some content before the break.

---

Content after the thematic break."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
title: Document with Thematic Break
---"#
        );
        assert_eq!(
            body,
            r#"Some content before the break.

---

Content after the thematic break."#
        );
    }

    #[test]
    fn handle_spaces_before_frontmatter() {
        let input = r#"   ---
title: Frontmatter with Leading Spaces
---
Body content here."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
title: Frontmatter with Leading Spaces
---"#
        );
        assert_eq!(body, "Body content here.");
    }

    #[test]
    fn handle_empty_frontmatter() {
        let input = r#"---
---
Body after empty frontmatter."#;
        let SplitResult { head, body } = split(input);
        assert_eq!(
            head,
            r#"---
---"#
        );
        assert_eq!(body, "Body after empty frontmatter.");
    }
}
