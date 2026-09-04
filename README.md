# AMP

AMP (stands for "Asuka's Markdown Parser") is a minimum markdown parser, written in TypeScript.

## Installation

```bash
# Using npm
npm install @asukawang/amp

# Using yarn
yarn add @asukawang/amp

# Using bun
bun add @asukawang/amp
```

## Usage

### Basic Usage

```typescript
import { Amp } from '@asukawang/amp';

// Create a new parser instance
const amp = new Amp();

// Parse markdown text into structured blocks
const markdownText = `
---
title: Hello world
---

# Heading 1
`;

const { frontmatter, blocks } = amp.parse(markdownText);
console.log(frontmatter); // { title: "Hello world" }
console.log(blocks); // [{ type: "heading", level: 1, body: [{ type: "textBody", style: "plain", value: "Heading 1" }] }]
```

### Extending with Custom Blocks

You can extend the parser with custom block types:

```typescript
import { Amp } from '@asukawang/amp';
import type { CustomBlock } from '@asukawang/amp';

// Define a custom block type
type StrikeThroughBlock = CustomBlock<'strikeThrough', { body: string }>;

const strikeThroughRegexp = new RegExp(/^~~(.+?)~~/);
const strikeThroughParser = (input: string): StrikeThroughBlock => {
  const match = input.match(strikeThroughRegexp);
  if (!match) {
    throw new Error('No match');
  }
  return {
    type: 'custom',
    customType: 'strikeThrough',
    body: match[1],
  };
};

// Extend the parser with the custom block
const amp = new Amp().extend([strikeThroughRegexp, strikeThroughParser]);

// Extract the extended block type
type ExtendedBlock = ReturnType<typeof amp.parse>['blocks'][number];
// ExtendedBlock = StrikeThroughBlock | HeadingBlock | ParagraphBlock | ...

// Now you can parse custom blocks alongside built-in blocks
const text = '~~This text is strikethrough~~';
const { blocks } = amp.parse(text);
console.log(blocks); // [{ type: "custom", customType: "strikeThrough", body: "This text is strikethrough" }]

// You can chain multiple extend() calls
const ampWithMultipleExtensions = new Amp()
  .extend([strikeThroughRegexp, strikeThroughParser])
  .extend([anotherRegexp, anotherParser]);
type MultipleExtendedBlock = ReturnType<typeof amp.parse>['blocks'][number];
// MultipleExtendedBlock = StrikeThroughBlock | AnotherBlock | HeadingBlock | ParagraphBlock | ...
```

## Built-in Block Types

### Paragraph

```ts
// Some text with **strong**, a [link](https://example.com) and a footnote[^1].
{
  type: 'paragraph';
  body: InlineContent[];
}

type InlineContent = TextBody | Link | FootnoteReference;
```

#### TextBody

```ts
{
  type: 'textBody';
  style: TextBodyStyle;
  value: string;
}
```

##### TextBodyStyle

- strong (\*\*text\*\*)
- italic (\_text\_, \*text\*)
- code (\`text\`)

#### Link

```ts
{
  type: 'link';
  body: TextBody[];
  url: string;
}
```

#### FootnoteReference

```ts
{
  type: 'footnoteReference';
  label: string;
}
```

### Heading

```ts
// # Heading 1
// ###### Heading 6
{
  type: 'heading';
  body: TextBody[];
  level: 1 | 2 | 3 | 4 | 5 | 6;
}
```

### List

```ts
// - item one
// - item two
//
// 1. item one
// 2. item two
{
  type: 'list';
  items: ListItem[];
  ordered: boolean;
}
```

For unordered list, only hyphen is supported (asterisks unsupported.)

#### ListItem

```ts
{
  type: 'listItem';
  body: (TextBody | Link)[];
}
```

### Quote

```ts
// > quoted text
// > more quoted text
{
  type: 'quote';
  body: (TextBody | Link)[];
}
```

### Image

```ts
// ![alt text](image url)(caption)
{
  type: 'image';
  url: string;
  altText: string;
  caption: string;
}
```

### Code

````ts
// ```js
// const foo = 'bar';
// ```
{
  type: 'code';
  lang?: string;
  body: string;
}
````

### Thematic break

```ts
// ---
{
  type: 'thematicBreak';
}
```

### Footnote

```ts
// [^1]: footnote text
// [^2]: another footnote
{
  type: 'footnote';
  items: FootnoteItem[];
}
```
#### FootnoteItem
```ts
{
  type: 'footnoteItem';
  label: string;
  body: InlineContent[];
}
```

# License

MIT
