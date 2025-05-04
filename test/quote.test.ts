import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/quote.test.md'),
  'utf-8',
);

describe('quote', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Quote test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about quote test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('quote-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      const quoteBlocks = blocks.filter((block) => block.type === 'quote');

      expect(quoteBlocks).toHaveLength(4);
      expect(quoteBlocks.every((block) => block.type === 'quote')).toBe(true);
    });

    test('single line quote content', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access quote properties
      const quoteBlocks = blocks.filter(
        (block) => block.type === 'quote',
      ) as import('../lib/definition').QuoteBlock[];

      // Single line quote content
      expect(quoteBlocks[0]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'single line quote',
          },
        ],
      });
    });

    test('multiple line quote content', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access quote properties
      const quoteBlocks = blocks.filter(
        (block) => block.type === 'quote',
      ) as import('../lib/definition').QuoteBlock[];

      // Multiple line quote content
      expect(quoteBlocks[1]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'multiple line quote 1\nmultiple line quote 2\nmultiple line quote 3\nmultiple line quote 4\nmultiple line quote 5',
          },
        ],
      });
    });

    test('styled text quote content', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access quote properties
      const quoteBlocks = blocks.filter(
        (block) => block.type === 'quote',
      ) as import('../lib/definition').QuoteBlock[];

      // Styled text quote content
      expect(quoteBlocks[2]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Lorem ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'ipsum',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' dolor sit ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'amet, consectetur adipiscing elit.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '\nLorem ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'ipsum',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' dolor sit ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'amet, consectetur adipiscing elit.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '\nLorem ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'ipsum',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' dolor sit ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'amet, consectetur adipiscing elit.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '\nLorem ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'ipsum',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' dolor sit ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'amet, consectetur adipiscing elit.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '\nLorem ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'ipsum',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' dolor ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'sit amet, consectetur adipiscing elit.',
              },
            ],
          },
        ],
      });
    });

    test('empty line in quote', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access quote properties
      const quoteBlocks = blocks.filter(
        (block) => block.type === 'quote',
      ) as import('../lib/definition').QuoteBlock[];

      // Empty line in quote content
      expect(quoteBlocks[3]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'empty line in quote 1\n\nempty line in quote 2\n\nempty line in quote 3',
          },
        ],
      });
    });
  });
});
