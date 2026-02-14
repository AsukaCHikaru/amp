struct SplitResult {
    head: String,
    body: String,
}

fn split(input: &str) -> SplitResult {
    SplitResult {
        head: "head".to_string(),
        body: "body".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_markdown_with_head_and_body() {
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
}
