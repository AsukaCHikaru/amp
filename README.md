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
{
  type: 'paragraph';
  body: (TextBody | Link)[];
}
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

### Heading

```ts
{
  type: 'heading';
  body: TextBody[];
  level: 1 | 2 | 3 | 4 | 5 | 6;
}
```

### List

```ts
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
{
  type: 'quote';
  body: (TextBody | Link)[];
}
```

### Image

```ts
{
  type: 'image';
  url: string;
  altText: string;
  caption: string;
}
```

### Code

```ts
{
  type: 'code';
  lang?: string;
  body: string;
}
```

### Thematic break

```ts
{
  type: 'thematicBreak';
}
```

# License

MIT
