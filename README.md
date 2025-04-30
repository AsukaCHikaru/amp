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

```typescript
import { parse } from '@asukawang/amp';

// Parse markdown text into structured blocks
const markdownText = `
---
title: Hello world
---

# Heading 1
`;

const { frontmatter, blocks } = parse(markdownText);
console.log(frontmatter) // { title: "Hello world" }
console.log(blocks) // [{ type: "heading", level: 1, body: [{ type: "textBody", style: "plain", value: "Heading 1" }] }]
```

## Supported block types

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
  body: (TextBody|Link)[];
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
