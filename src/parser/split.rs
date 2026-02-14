use regex::Regex;

struct SplitResult {
    head: String,
    body: String,
}

fn split(input: &str) -> SplitResult {
    let trimmed = input.trim();
    let pattern = Regex::new(r"^(---\n[\s\S]*?---)\n*").expect("Invalid regex");

    match pattern.captures(trimmed) {
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
