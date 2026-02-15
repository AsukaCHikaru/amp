use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

static FRONTMATTER_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"---\n+([\s\S]+)---").expect("Invalid regex"));
static LINE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(.+?):\s(.+)").expect("Invalid regex"));

fn parse_frontmatter(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    match FRONTMATTER_PATTERN.captures(input.trim()) {
        Some(captured) => {
            let raw_frontmatter = captured.get(1).unwrap().as_str().to_string();
            raw_frontmatter
                .split("\n")
                .filter(|line| !(line.is_empty()))
                .map(|line| match LINE_PATTERN.captures(line) {
                    Some(line_captured) => {
                        let key = line_captured.get(1).unwrap().as_str().to_string();
                        let value = line_captured.get(2).unwrap().as_str().to_string();
                        (key, value)
                    }
                    None => ("".to_string(), "".to_string()),
                })
                .for_each(|(key, value)| {
                    map.insert(key, value);
                });

            map
        }
        None => map,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_frontmatter_with_string_values() {
        let input = r#"---
title: Hello World
description: This is a test
author: John Doe
---

# Content starts here"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("description").unwrap(), "This is a test");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn handle_frontmatter_with_number_values() {
        let input = r#"---
version: 1
count: 42
rating: 4.5
---

Some content"#;
        let result = parse_frontmatter(input);

        assert_eq!(result.get("version").unwrap(), "1");
        assert_eq!(result.get("count").unwrap(), "42");
        assert_eq!(result.get("rating").unwrap(), "4.5");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn handle_frontmatter_with_string_array_values() {
        let input = r#"---
tags: javascript, typescript, react
categories: programming, web
---

Content here"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("tags").unwrap(), "javascript, typescript, react");
        assert_eq!(result.get("categories").unwrap(), "programming, web");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_number_array_values() {
        let input = r#"---
scores: 98, 87, 92
ratings: 4.5, 3.8, 5.0
---

Content"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("scores").unwrap(), "98, 87, 92");
        assert_eq!(result.get("ratings").unwrap(), "4.5, 3.8, 5.0");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_mixed_values() {
        let input = r#"---
title: Mixed Types Example
version: 2
tags: test, example
counts: 1, 2, 3
---

# Content"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Mixed Types Example");
        assert_eq!(result.get("version").unwrap(), "2");
        assert_eq!(result.get("tags").unwrap(), "test, example");
        assert_eq!(result.get("counts").unwrap(), "1, 2, 3");
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn handle_empty_frontmatter() {
        let input = r#"---
---

Content"#;
        let result = parse_frontmatter(input);
        assert!(result.is_empty());
    }

    #[test]
    fn handle_no_frontmatter_in_input() {
        let input = r#"# No frontmatter here
    
Just regular content"#;
        let result = parse_frontmatter(input);
        assert!(result.is_empty());
    }

    #[test]
    fn handle_frontmatter_with_whitespace() {
        let input = r#"---
  title:    Spaced Content   
  tags:     one ,  two  
---

Content"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Spaced Content");
        assert_eq!(result.get("tags").unwrap(), "one ,  two");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn ignore_content_after_frontmatter() {
        let input = r#"---
title: Just the frontmatter
---

# This should be ignored
And this too"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Just the frontmatter");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn handle_malformed_frontmatter() {
        let input = r#"--
title: Missing a dash
--

Content"#;
        let result = parse_frontmatter(input);
        assert!(result.is_empty())
    }

    #[test]
    fn handle_frontmatter_without_closing_delimiter() {
        let input = r#"---
title: No closing
author: Someone

Content"#;
        let result = parse_frontmatter(input);
        assert!(result.is_empty());
    }

    #[test]
    fn handle_quotes_in_frontmatter() {
        let input = r#"---
title: "Title with quotes"
description: 'Description with "quotes" and 'apostrophes''
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Title with quotes");
        assert_eq!(
            result.get("description").unwrap(),
            "Description with \"quotes\" and 'apostrophes'"
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_colons() {
        let input = r#"---
title: "Title: With Colons"
description: "Description: With Colons"
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Title: With Colons");
        assert_eq!(
            result.get("description").unwrap(),
            "Description: With Colons"
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_empty_line() {
        let input = r#"---
title: Hello World

author: John Doe
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_line_without_colon() {
        let input = r#"---
title: Hello World
description
author: John Doe
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_empty_value() {
        let input = r#"---
title: Hello World
description:
author: John Doe
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("description").unwrap(), "");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn handle_frontmatter_with_key_of_only_whitespaces() {
        let input = r#"---
title: Hello World
: This is a test
author: John Doe
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn handle_frontmatter_with_value_of_only_whitespaces() {
        let input = r#"---
title: Hello World
description:     
author: John Doe
---"#;
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title").unwrap(), "Hello World");
        assert_eq!(result.get("description").unwrap(), "");
        assert_eq!(result.get("author").unwrap(), "John Doe");
        assert_eq!(result.len(), 3);
    }
}
