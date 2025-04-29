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
# Heading 1

This is a paragraph with **bold** and *italic* text.

- List item 1
- List item 2

> This is a quote

\`\`\`typescript
const code = "This is a code block";
\`\`\`

---
`;

const { blocks } = parse(markdownText);
console.log(blocks);
```

## Supported block types

- Heading
  - h1 ~ h6
- Styled texts
  - italic
  - bold
  - code
  - link
- Quote
- List
  - ordered (asterisks unsupported)
  - unordered
- Code
- Thematic break

# License

MIT
