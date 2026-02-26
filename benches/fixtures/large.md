---
title: Large benchmark document
description: A comprehensive markdown document for stress testing the parser
author: Benchmark Suite
version: 1.0.0
tags: benchmark, stress-test, large, comprehensive
---

# Chapter 1: Introduction

This is the opening paragraph of a large document designed to **stress test** the markdown parser. It contains _various inline styles_, `code snippets`, and [hyperlinks](https://example.com) throughout.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas pharetra sem in ante varius, id mollis sapien varius. Nunc et rutrum arcu. Pellentesque habitant morbi tristique senectus et netus.

## 1.1 Background

Suspendisse potenti. Cras **strongly emphasized** content here with _italicized words_ and `code snippets` mixed in for good measure. Duis mollis, est non commodo luctus, nisi erat porttitor ligula.

Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus. Etiam porta sem malesuada magna mollis euismod.

### 1.1.1 Details

- First item with **bold** text and more content
- Second item with _italic_ text and additional words
- Third item with `code` text and extra details
- Fourth item with [link](https://example.com) and surrounding text

### 1.1.2 More Details

> This is a blockquote with **bold** and _italic_ and `code` and [link](https://example.com) inside it. It spans a decent length to test inline parsing within quotes.

---

## 1.2 Motivation

Donec sed odio dui. Nullam quis risus eget urna mollis ornare vel eu leo. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.

```typescript
interface Parser {
  parse(input: string): ParseResult;
  extend(block: CustomBlock): Parser;
}

class Amp implements Parser {
  parse(input: string): ParseResult {
    const { head, body } = split(input);
    return { frontmatter: parseFrontmatter(head), blocks: parseBlocks(body) };
  }
}
```

Aenean lacinia bibendum nulla sed consectetur. Praesent commodo cursus magna, vel scelerisque nisl consectetur et.

# Chapter 2: Architecture

The architecture of the system is built around a **modular parsing pipeline** that processes markdown in distinct phases.

## 2.1 Splitting Phase

The first phase splits the input into _frontmatter_ and _body_ sections. This is handled by the `split` function.

### 2.1.1 Frontmatter Parsing

Frontmatter is parsed as key-value pairs separated by colons. Each value can contain **commas**, _special characters_, and `inline code`.

- Key-value pairs are separated by newlines
- Values are trimmed of whitespace
- Multiple values can be comma-separated
- Empty values are allowed

### 2.1.2 Body Splitting

The body is split into blocks by double newlines. Each block is then matched against registered patterns.

> The splitting phase is critical for performance. A fast splitter means the rest of the pipeline can focus on detailed parsing without worrying about block boundaries.

---

## 2.2 Block Parsing Phase

Each block is matched against a series of regular expressions in priority order.

```rust
pub fn parse_blocks(input: &str) -> Vec<Block> {
    let raw_blocks = split_blocks(input);
    raw_blocks
        .iter()
        .map(|raw| parse_single_block(raw))
        .collect()
}

fn parse_single_block(input: &str) -> Block {
    if is_heading(input) {
        parse_heading(input)
    } else if is_code(input) {
        parse_code(input)
    } else {
        parse_paragraph(input)
    }
}
```

### 2.2.1 Heading Blocks

Headings are identified by leading `#` characters. The parser supports levels 1 through 6.

### 2.2.2 Code Blocks

Code blocks are delimited by triple backticks. An optional language identifier follows the opening backticks.

### 2.2.3 List Blocks

- Unordered lists use `-` as the bullet character
- Each item can contain **inline styles**
- Nested lists are not currently supported
- Items are parsed for inline content

### 2.2.4 Quote Blocks

> Quotes are prefixed with `>` and can contain any inline content including **bold**, _italic_, `code`, and [links](https://example.com).

### 2.2.5 Image Blocks

[screenshot](screenshot.png)(A screenshot of the application)

### 2.2.6 Thematic Breaks

---

## 2.3 Inline Parsing

Inline content is parsed within each block. The parser recognizes **bold**, _italic_, `code`, and [links](https://example.com).

Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor. Duis mollis, est non commodo luctus, nisi erat porttitor ligula, eget lacinia odio sem nec elit.

# Chapter 3: Implementation

The implementation follows a **test-driven development** approach with comprehensive test coverage.

## 3.1 Rust Core

The Rust implementation provides the core parsing logic compiled to WebAssembly.

### 3.1.1 Type System

```rust
pub struct ParseResult {
    pub frontmatter: HashMap<String, String>,
    pub blocks: Vec<Block>,
}

pub enum Block {
    Heading(HeadingBlock),
    Paragraph(ParagraphBlock),
    Code(CodeBlock),
    List(ListBlock),
    Quote(QuoteBlock),
    Image(ImageBlock),
    ThematicBreak(ThematicBreak),
}
```

### 3.1.2 WASM Bindings

The WASM bindings use `wasm-bindgen` and `tsify-next` to generate TypeScript types automatically.

- `#[wasm_bindgen]` marks structs and methods for export
- `#[tsify(into_wasm_abi)]` enables automatic TS type generation
- `serde` handles serialization between Rust and JavaScript

## 3.2 TypeScript Wrapper

The TypeScript wrapper loads the WASM module and re-exports a clean API.

### 3.2.1 Module Loading

```typescript
import { readFileSync } from 'fs';
import { initSync } from './pkg/amp';

const wasmBytes = readFileSync(new URL('./pkg/amp_bg.wasm', import.meta.url));
initSync(wasmBytes);
```

### 3.2.2 API Surface

The public API exposes:

- `Amp` class with `parse` method
- All block types as TypeScript interfaces
- `ParseResult` type for parse output
- `extend` method for custom blocks

---

## 3.3 Testing Strategy

Tests cover both unit and integration levels:

- Unit tests for individual parsers (heading, paragraph, code, etc.)
- Integration tests for full document parsing
- Snapshot tests for complex documents
- Cross-implementation tests comparing TS and Rust output

# Chapter 4: Performance

Performance is a key consideration for the parser, especially when processing large documents.

## 4.1 Benchmarks

The benchmark suite tests both implementations across different document sizes.

### 4.1.1 Small Documents

Small documents (under 20 lines) test the baseline overhead of parser initialization and single-pass processing.

### 4.1.2 Medium Documents

Medium documents (50-100 lines) represent typical blog posts and documentation pages.

### 4.1.3 Large Documents

Large documents (500+ lines) stress test the parser's ability to handle complex content efficiently.

## 4.2 Optimization Techniques

Several optimization techniques are employed:

- **Regex compilation**: Patterns are compiled once and reused
- **Zero-copy parsing**: Where possible, slices reference the original input
- **Block-level parallelism**: Blocks can be parsed independently
- **Inline caching**: Common inline patterns are cached

> Performance should be measured in real-world scenarios, not just microbenchmarks. The benchmark suite includes realistic documents to ensure meaningful results.

---

## 4.3 Memory Usage

Memory usage is kept low through careful allocation strategies:

- Stack allocation for small types
- Minimal heap allocation during parsing
- String slices instead of owned strings where safe
- Efficient enum representations for block types

# Chapter 5: Future Work

The parser will continue to evolve with new features and optimizations.

## 5.1 Planned Features

- Table support with alignment
- Footnote references
- Task lists with checkboxes
- Definition lists

## 5.2 Extension System

The `extend` method allows users to add custom block types:

```typescript
const parser = new Amp();
parser.extend([customRegex, customParser]);
const result = parser.parse(input);
```

### 5.2.1 Custom Block Types

Custom blocks must implement the `CustomBlock` interface:

- A unique `type` string identifier
- A regex pattern for matching
- A parser function that returns the block data

### 5.2.2 Priority System

- Custom blocks are checked before built-in blocks
- This allows overriding default behavior
- Priority is determined by registration order

## 5.3 Build Pipeline

The build pipeline automates:

- Rust compilation to WASM
- TypeScript type generation
- Bundle creation for multiple targets
- NPM package publishing

> The goal is a fully automated CI/CD pipeline that handles everything from code push to npm publish.

---

# Appendix A: Block Reference

## Headings

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

## Paragraphs

This is a plain paragraph without any inline styles.

This is a paragraph with **bold**, _italic_, `code`, and [link](https://example.com).

This is a paragraph with **multiple bold segments** and _multiple italic segments_ and `multiple code segments`.

## Lists

- Simple item one
- Simple item two
- Simple item three

- Item with **bold** content
- Item with _italic_ content
- Item with `code` content
- Item with [link](https://example.com) content

## Quotes

> Simple quote without inline styles.

> Quote with **bold**, _italic_, `code`, and [link](https://example.com).

## Code Blocks

```javascript
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log(fibonacci(10));
```

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

print(fibonacci(10))
```

```rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("{}", fibonacci(10));
}
```

## Images

[photo](photo.jpg)(A beautiful landscape)

[diagram](diagram.svg)(System architecture diagram)

## Thematic Breaks

---

---

---

# Appendix B: Inline Styles Reference

Paragraph with just **bold** text.

Paragraph with just _italic_ text.

Paragraph with just `code` text.

Paragraph with just [link](https://example.com).

Paragraph with **bold** and _italic_ together.

Paragraph with **bold** and `code` together.

Paragraph with **bold** and [link](https://example.com) together.

Paragraph with _italic_ and `code` together.

Paragraph with _italic_ and [link](https://example.com) together.

Paragraph with `code` and [link](https://example.com) together.

Paragraph with **bold**, _italic_, `code`, and [link](https://example.com) all together in one line.

The end of the benchmark document.
