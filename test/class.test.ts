import { describe, expect, test } from 'bun:test';
import { Amp } from '../lib/parser';
import type { CustomBlock } from '../lib/definition';

// Shared test utilities
const strikeThroughRegexp = new RegExp(/^~~(.+?)~~/);
const strikeThroughParser = (input: string): CustomBlock => {
  const match = input.match(strikeThroughRegexp);
  if (!match) {
    throw new Error('No match');
  }
  return {
    type: 'strikeThrough',
    body: match[1],
  };
};

const highlightRegexp = new RegExp(/^==(.+?)==/);
const highlightParser = (input: string): CustomBlock => {
  const match = input.match(highlightRegexp);
  if (!match) {
    throw new Error('No match');
  }
  return {
    type: 'highlight',
    body: match[1],
  };
};

describe('Amp class', () => {
  test('instantiate', () => {
    const amp = new Amp();
    expect(amp).toBeDefined();
    expect(amp.parse).toBeFunction();
    expect(amp.extend).toBeFunction();
  });
  describe('extend method', () => {
    test('can extend with custom block', () => {
      const amp = new Amp();
      amp.extend([[strikeThroughRegexp, strikeThroughParser]]);

      const input = '~~strikethrough text~~';
      const { blocks } = amp.parse(input);

      expect(blocks).toHaveLength(1);
      expect(blocks[0]).toMatchObject({
        type: 'strikeThrough',
        body: 'strikethrough text',
      });
    });

    test('can extend with multiple custom blocks', () => {
      const amp = new Amp();
      amp.extend([
        [strikeThroughRegexp, strikeThroughParser],
        [highlightRegexp, highlightParser],
      ]);

      const input1 = '~~strikethrough text~~';
      const { blocks: blocks1 } = amp.parse(input1);
      expect(blocks1).toHaveLength(1);
      expect(blocks1[0]).toMatchObject({
        type: 'strikeThrough',
        body: 'strikethrough text',
      });

      const input2 = '==highlighted text==';
      const { blocks: blocks2 } = amp.parse(input2);
      expect(blocks2).toHaveLength(1);
      expect(blocks2[0]).toMatchObject({
        type: 'highlight',
        body: 'highlighted text',
      });
    });

    test('custom blocks work alongside built-in blocks', () => {
      const amp = new Amp();
      amp.extend([[strikeThroughRegexp, strikeThroughParser]]);

      const input = '# Heading\n\n~~strikethrough text~~\n\nThis is a paragraph';
      const { blocks } = amp.parse(input);

      expect(blocks).toHaveLength(3);
      expect(blocks[0].type).toBe('heading');
      expect(blocks[1]).toMatchObject({
        type: 'strikeThrough',
        body: 'strikethrough text',
      });
      expect(blocks[2].type).toBe('paragraph');
    });

    test('custom blocks work with complex markdown document', () => {
      const amp = new Amp();
      amp.extend([
        [strikeThroughRegexp, strikeThroughParser],
        [highlightRegexp, highlightParser],
      ]);

      const input = `---
title: Test Document
author: Test Author
---

# Main Heading

This is an introductory paragraph with **bold** and *italic* text.

## Features Section

~~This feature is deprecated~~

> This is a quote block with some important information.
> It spans multiple lines.

### List of Items

- First item
- Second item
- Third item

==This is highlighted text==

1. Numbered item one
2. Numbered item two
3. Numbered item three

#### Code Example

\`\`\`javascript
const x = 5;
console.log(x);
\`\`\`

~~Another deprecated feature~~

![Alt text](image.jpg)(This is a caption)

---

##### Final Notes

This is the final paragraph before we end.

==Important note at the end==`;

      const { frontmatter, blocks } = amp.parse(input);

      // Verify frontmatter
      expect(frontmatter['title']).toBe('Test Document');
      expect(frontmatter['author']).toBe('Test Author');

      // Verify blocks structure
      expect(blocks.length).toBeGreaterThan(10);

      // Find and verify custom blocks
      const strikeThroughBlocks = blocks.filter(
        (block) => block.type === 'strikeThrough',
      );
      expect(strikeThroughBlocks).toHaveLength(2);
      expect(strikeThroughBlocks[0]).toMatchObject({
        type: 'strikeThrough',
        body: 'This feature is deprecated',
      });
      expect(strikeThroughBlocks[1]).toMatchObject({
        type: 'strikeThrough',
        body: 'Another deprecated feature',
      });

      const highlightBlocks = blocks.filter(
        (block) => block.type === 'highlight',
      );
      expect(highlightBlocks).toHaveLength(2);
      expect(highlightBlocks[0]).toMatchObject({
        type: 'highlight',
        body: 'This is highlighted text',
      });
      expect(highlightBlocks[1]).toMatchObject({
        type: 'highlight',
        body: 'Important note at the end',
      });

      // Verify built-in blocks are still working
      const headingBlocks = blocks.filter((block) => block.type === 'heading');
      expect(headingBlocks.length).toBeGreaterThanOrEqual(5);
      expect(headingBlocks[0]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Main Heading',
          },
        ],
      });
      expect(headingBlocks[1]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Features Section',
          },
        ],
      });

      const quoteBlocks = blocks.filter((block) => block.type === 'quote');
      expect(quoteBlocks).toHaveLength(1);
      expect(quoteBlocks[0]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'This is a quote block with some important information.\nIt spans multiple lines.',
          },
        ],
      });

      const listBlocks = blocks.filter((block) => block.type === 'list');
      expect(listBlocks).toHaveLength(2);
      expect(listBlocks[0]).toMatchObject({
        type: 'list',
        ordered: false,
        items: [
          {
            type: 'listItem',
            body: [{ type: 'textBody', style: 'plain', value: 'First item' }],
          },
          {
            type: 'listItem',
            body: [{ type: 'textBody', style: 'plain', value: 'Second item' }],
          },
          {
            type: 'listItem',
            body: [{ type: 'textBody', style: 'plain', value: 'Third item' }],
          },
        ],
      });
      expect(listBlocks[1]).toMatchObject({
        type: 'list',
        ordered: true,
        items: [
          {
            type: 'listItem',
            body: [
              { type: 'textBody', style: 'plain', value: 'Numbered item one' },
            ],
          },
          {
            type: 'listItem',
            body: [
              { type: 'textBody', style: 'plain', value: 'Numbered item two' },
            ],
          },
          {
            type: 'listItem',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'Numbered item three',
              },
            ],
          },
        ],
      });

      const codeBlocks = blocks.filter((block) => block.type === 'code');
      expect(codeBlocks).toHaveLength(1);
      expect(codeBlocks[0]).toMatchObject({
        type: 'code',
        lang: 'javascript',
        body: 'const x = 5;\nconsole.log(x);',
      });

      const imageBlocks = blocks.filter((block) => block.type === 'image');
      expect(imageBlocks).toHaveLength(1);
      expect(imageBlocks[0]).toMatchObject({
        type: 'image',
        url: 'image.jpg',
        altText: 'Alt text',
        caption: 'This is a caption',
      });

      const thematicBreakBlocks = blocks.filter(
        (block) => block.type === 'thematicBreak',
      );
      expect(thematicBreakBlocks).toHaveLength(1);
      expect(thematicBreakBlocks[0]).toMatchObject({
        type: 'thematicBreak',
      });

      // Verify paragraph with styled text
      const paragraphBlocks = blocks.filter(
        (block) => block.type === 'paragraph',
      );
      expect(paragraphBlocks.length).toBeGreaterThanOrEqual(2);
      expect(paragraphBlocks[0]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is an introductory paragraph with ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'bold',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' and ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'italic',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' text.',
          },
        ],
      });
    });
  });
});
