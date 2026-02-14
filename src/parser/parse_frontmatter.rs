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
                        println!("{}, {}", key, value);
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
    }
}
