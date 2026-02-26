---
title: Medium document
description: A moderately sized markdown file for benchmarking
tags: benchmark, test, medium
---

# Introduction

This is the introduction paragraph with **bold text**, _italic text_, `inline code`, and a [link](https://example.com).

## Section One

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas pharetra sem in ante varius, id mollis sapien varius. Nunc et rutrum arcu. Pellentesque habitant morbi tristique senectus et netus.

Suspendisse potenti. Cras **strongly emphasized** content here with _italicized words_ and `code snippets` mixed in for good measure.

### Subsection 1.1

- First item with **bold** text
- Second item with _italic_ text
- Third item with `code` text
- Fourth item with [link](https://example.com)

### Subsection 1.2

> This is a blockquote with **bold** and _italic_ and `code` and [link](https://example.com) inside it.

---

## Section Two

```rust
fn main() {
    println!("Hello, world!");
    let x = 42;
    let y = x * 2;
    println!("The answer is {}", y);
}
```

Another paragraph here with some more text. This helps test how the parser handles multiple blocks in sequence.

### Subsection 2.1

[diagram](architecture.png)(System architecture overview)

Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus.

### Subsection 2.2

- List item alpha
- List item beta
- List item gamma
- List item delta

## Section Three

Duis mollis, est non commodo luctus, nisi erat porttitor ligula, eget lacinia odio sem nec elit. Cras mattis consectetur purus sit amet fermentum.

> Another blockquote to test parsing of multiple quote blocks in the same document.

---

#### Deep Heading

##### Deeper Heading

###### Deepest Heading

Final paragraph with **all** _the_ `inline` [styles](https://example.com) combined.
