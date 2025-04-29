import type {
  Block,
  HeadingBlock,
  ImageBlock,
  ListBlock,
  QuoteBlock,
} from './definition';
import {
  parse,
  parseBlock,
  parseHeadingBlock,
  parseImageBlock,
  parseListBlock,
  parseQuoteBlock,
} from './parser';
import { describe, expect, test } from 'bun:test';

describe('parseImageBlock', () => {
  test('parses basic image with alt text, URL, and caption', () => {
    const input = '![Sample Alt Text](/images/sample.jpg)(This is a caption)';
    const expected = {
      type: 'image',
      url: '/images/sample.jpg',
      altText: 'Sample Alt Text',
      caption: 'This is a caption',
    } satisfies ImageBlock;
    expect(parseImageBlock(input)).toEqual(expected);
  });

  test('parses image with spaces in URL', () => {
    const input = '![Alt text](/path/to/image with spaces.jpg)(Caption)';
    const expected = {
      type: 'image',
      url: '/path/to/image with spaces.jpg',
      altText: 'Alt text',
      caption: 'Caption',
    } satisfies ImageBlock;
    expect(parseImageBlock(input)).toEqual(expected);
  });

  test('parses image with empty alt text', () => {
    const input = '![](/images/no-alt.jpg)(Image without alt text)';
    const expected = {
      type: 'image',
      url: '/images/no-alt.jpg',
      altText: '',
      caption: 'Image without alt text',
    } satisfies ImageBlock;
    expect(parseImageBlock(input)).toEqual(expected);
  });

  test('parses image without caption', () => {
    const input = '![Alt text only](/images/no-caption.jpg)';
    const expected = {
      type: 'image',
      url: '/images/no-caption.jpg',
      altText: 'Alt text only',
      caption: '',
    } satisfies ImageBlock;
    expect(parseImageBlock(input)).toEqual(expected);
  });
});

describe('parseHeadingBlock', () => {
  test('parses H1 heading', () => {
    const input = '# Heading 1';
    const expected: HeadingBlock = {
      type: 'heading',
      level: 1,
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Heading 1',
        },
      ],
    };
    expect(parseHeadingBlock(input)).toEqual(expected);
  });

  test('parses H2 heading', () => {
    const input = '## Heading 2';
    const expected: HeadingBlock = {
      type: 'heading',
      level: 2,
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Heading 2',
        },
      ],
    };
    expect(parseHeadingBlock(input)).toEqual(expected);
  });

  test('parses H6 heading', () => {
    const input = '###### Heading 6';
    const expected: HeadingBlock = {
      type: 'heading',
      level: 6,
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Heading 6',
        },
      ],
    };
    expect(parseHeadingBlock(input)).toEqual(expected);
  });

  test('parses heading with styled text', () => {
    const input = '# Heading with **strong** and *italic* text';
    const expected: HeadingBlock = {
      type: 'heading',
      level: 1,
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Heading with ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'strong',
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
          value: ' text',
        },
      ],
    };
    expect(parseHeadingBlock(input)).toEqual(expected);
  });
});

describe('parseQuoteBlock', () => {
  test('parses simple quote', () => {
    const input = '> This is a quote';
    const expected: QuoteBlock = {
      type: 'quote',
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'This is a quote',
        },
      ],
    };
    expect(parseQuoteBlock(input)).toEqual(expected);
  });

  test('parses quote with styled text', () => {
    const input = '> Quote with **strong** and *italic* text';
    const expected: QuoteBlock = {
      type: 'quote',
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Quote with ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'strong',
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
          value: ' text',
        },
      ],
    };
    expect(parseQuoteBlock(input)).toEqual(expected);
  });
});

describe('parseListBlock', () => {
  test('parses single unordered list with hyphens', () => {
    const input = '- Item 1';
    const expected: ListBlock = {
      type: 'list',
      ordered: false,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 1',
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });
  test('parses multiple unordered list with hyphens', () => {
    const input = `- Item 1
- Item 2`;
    const expected: ListBlock = {
      type: 'list',
      ordered: false,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 1',
            },
          ],
        },
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 2',
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });

  test('parses single ordered list', () => {
    const input = '1. Item 1';
    const expected: ListBlock = {
      type: 'list',
      ordered: true,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 1',
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });
  test('parses multiple ordered list', () => {
    const input = `1. Item 1
2. Item 2`;
    const expected: ListBlock = {
      type: 'list',
      ordered: true,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 1',
            },
          ],
        },
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item 2',
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });

  test('parses list with styled text', () => {
    const input = '- Item with **strong** and *italic* text';
    const expected: ListBlock = {
      type: 'list',
      ordered: false,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item with ',
            },
            {
              type: 'textBody',
              style: 'strong',
              value: 'strong',
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
              value: ' text',
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });

  test('parses list with link', () => {
    const input = `- Item with [link](https://example.com)
- Item with [link2](https://example.com)`;
    const expected: ListBlock = {
      type: 'list',
      ordered: false,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item with ',
            },
            {
              type: 'link',
              url: 'https://example.com',
              body: [
                {
                  type: 'textBody',
                  style: 'plain',
                  value: 'link',
                },
              ],
            },
          ],
        },
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'Item with ',
            },
            {
              type: 'link',
              url: 'https://example.com',
              body: [
                {
                  type: 'textBody',
                  style: 'plain',
                  value: 'link2',
                },
              ],
            },
          ],
        },
      ],
    };
    expect(parseListBlock(input)).toEqual(expected);
  });
});

describe('parseBlock', () => {
  test('parses heading block', () => {
    const input = '# Heading 1';
    const expected: Block = {
      type: 'heading',
      level: 1,
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Heading 1',
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses quote block', () => {
    const input = '> This is a quote';
    const expected: Block = {
      type: 'quote',
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'This is a quote',
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses unordered list block', () => {
    const input = `- List item 1
- List item 2`;
    const expected: Block = {
      type: 'list',
      ordered: false,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'List item 1',
            },
          ],
        },
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'List item 2',
            },
          ],
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses ordered list block', () => {
    const input = `1. List item 1
2. List item 2`;
    const expected: Block = {
      type: 'list',
      ordered: true,
      items: [
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'List item 1',
            },
          ],
        },
        {
          type: 'listItem',
          body: [
            {
              type: 'textBody',
              style: 'plain',
              value: 'List item 2',
            },
          ],
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses paragraph block by default', () => {
    const input = 'This is a paragraph';
    const expected: Block = {
      type: 'paragraph',
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'This is a paragraph',
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses paragraph with styled text', () => {
    const input = 'Paragraph with **strong** and *italic* text';
    const expected: Block = {
      type: 'paragraph',
      body: [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Paragraph with ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'strong',
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
          value: ' text',
        },
      ],
    };
    expect(parseBlock(input)).toEqual(expected);
  });

  test('parses image block', () => {
    const input = '![Alt text](/path/to/image.jpg)(Image caption)';
    const expected: Block = {
      type: 'image',
      url: '/path/to/image.jpg',
      altText: 'Alt text',
      caption: 'Image caption',
    };
    expect(parseBlock(input)).toEqual(expected);
  });
});

describe('parse', () => {
  test('parses multiple blocks', () => {
    const input = '# Heading 1\n> This is a quote\nThis is a paragraph';
    const expected: Block[] = [
      {
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Heading 1',
          },
        ],
      },
      {
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is a quote',
          },
        ],
      },
      {
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is a paragraph',
          },
        ],
      },
    ];
    expect(parse(input)).toEqual(expected);
  });

  test('parses empty lines as empty paragraphs', () => {
    const input = '# Heading 1\n\n> This is a quote';
    const expected: Block[] = [
      {
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Heading 1',
          },
        ],
      },
      {
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is a quote',
          },
        ],
      },
    ];
    expect(parse(input)).toEqual(expected);
  });

  test('parses complex document with mixed blocks', () => {
    const input =
      '# Heading 1\n## Heading 2\n> Quote with **strong** text\nParagraph with *italic* text';
    const expected: Block[] = [
      {
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Heading 1',
          },
        ],
      },
      {
        type: 'heading',
        level: 2,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Heading 2',
          },
        ],
      },
      {
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Quote with ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'strong',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' text',
          },
        ],
      },
      {
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Paragraph with ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'italic',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' text',
          },
        ],
      },
    ];
    expect(parse(input)).toEqual(expected);
  });

  test('parses document with image block', () => {
    const input =
      '# Document with Image\n![Image alt text](/path/to/image.jpg)(Image caption)\nText after image';
    const expected: Block[] = [
      {
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Document with Image',
          },
        ],
      },
      {
        type: 'image',
        url: '/path/to/image.jpg',
        altText: 'Image alt text',
        caption: 'Image caption',
      },
      {
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Text after image',
          },
        ],
      },
    ];
    expect(parse(input)).toEqual(expected);
  });
});
