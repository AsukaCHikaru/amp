use std::collections::HashMap;

fn parse_frontmatter(input: &str) -> HashMap<String, String> {
    let map = HashMap::new();

    map
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
    }
}
