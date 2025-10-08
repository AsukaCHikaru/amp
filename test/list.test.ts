import { readFileSync } from 'fs';
import { describe, expect, test } from 'vitest';
import { join } from 'path';
import { Amp } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/list.test.md'),
  'utf-8',
);

describe('list', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const amp = new Amp();
      const { frontmatter } = amp.parse(markdownContent);

      expect(frontmatter['title']).toBe('List test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about list test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('list-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Filter only list blocks
      const listBlocks = blocks.filter((block) => block.type === 'list');

      // We expect 5 list blocks to be parsed correctly
      expect(listBlocks).toHaveLength(5);
    });

    test('should identify ordered and unordered lists correctly', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Single unordered list
      expect(listBlocks[0].ordered).toBe(false);

      // Single ordered list
      expect(listBlocks[1].ordered).toBe(true);

      // Multi-item unordered list
      expect(listBlocks[2].ordered).toBe(false);

      // Multi-item ordered list
      expect(listBlocks[3].ordered).toBe(true);

      // Styled text list
      expect(listBlocks[4].ordered).toBe(false);
    });

    test('should parse list items correctly', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Single unordered list should have 1 item
      expect(listBlocks[0].items).toHaveLength(1);

      // Single ordered list should have 1 item
      expect(listBlocks[1].items).toHaveLength(1);

      // Multi-item unordered list should have 5 items
      expect(listBlocks[2].items).toHaveLength(5);

      // Multi-item ordered list should have 5 items
      expect(listBlocks[3].items).toHaveLength(5);

      // Styled text list should have 6 items
      expect(listBlocks[4].items).toHaveLength(6);
    });

    test('single unordered list content', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Single unordered list item content
      expect(listBlocks[0].items[0]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'single unordered',
          },
        ],
      });
    });

    test('single ordered list content', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Single ordered list item content
      expect(listBlocks[1].items[0]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'single ordered',
          },
        ],
      });
    });

    test('multi-item unordered list content', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Multi-item unordered list content
      expect(listBlocks[2].items[0]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'unordered 1',
          },
        ],
      });

      expect(listBlocks[2].items[1]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'unordered 2',
          },
        ],
      });

      expect(listBlocks[2].items[2]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'unordered 3',
          },
        ],
      });

      expect(listBlocks[2].items[3]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'unordered 4',
          },
        ],
      });

      expect(listBlocks[2].items[4]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'unordered 5',
          },
        ],
      });
    });

    test('multi-item ordered list content', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Multi-item ordered list content
      expect(listBlocks[3].items[0]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'ordered 1',
          },
        ],
      });

      expect(listBlocks[3].items[1]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'ordered 2',
          },
        ],
      });

      expect(listBlocks[3].items[2]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'ordered 3',
          },
        ],
      });

      expect(listBlocks[3].items[3]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'ordered 4',
          },
        ],
      });

      expect(listBlocks[3].items[4]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'ordered 5',
          },
        ],
      });
    });

    test('styled text list content', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      // Type assertion for blocks to access list properties
      const listBlocks = blocks.filter(
        (block) => block.type === 'list',
      ) as import('../lib/definition').ListBlock[];

      // Plain text item
      expect(listBlocks[4].items[0]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
          },
        ],
      });

      // Bold text item
      expect(listBlocks[4].items[1]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'strong',
            value: 'Lorem',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ipsum dolor ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'sit amet, consectetur adipiscing elit.',
          },
        ],
      });

      // Italic text item (using underscore)
      expect(listBlocks[4].items[2]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'italic',
            value: 'Lorem',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ipsum dolor ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'sit amet, consectetur adipiscing elit.',
          },
        ],
      });

      // Italic text item (using asterisk)
      expect(listBlocks[4].items[3]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'italic',
            value: 'Lorem',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ipsum dolor ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'sit amet, consectetur adipiscing elit.',
          },
        ],
      });

      // Code text item
      expect(listBlocks[4].items[4]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'textBody',
            style: 'code',
            value: 'Lorem',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ipsum dolor ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'sit amet, consectetur adipiscing elit.',
          },
        ],
      });

      // Link item
      expect(listBlocks[4].items[5]).toMatchObject({
        type: 'listItem',
        body: [
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'Lorem',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ipsum dolor ',
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
  });
});
