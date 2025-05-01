import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/image.test.md'),
  'utf-8',
);

describe('image', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Image test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about image test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('image-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      // Filter only image blocks
      const imageBlocks = blocks.filter(block => block.type === 'image');
      
      // We expect 5 image blocks to be parsed correctly
      expect(imageBlocks).toHaveLength(7);
    });

    test('block content', () => {
      const { blocks } = parse(markdownContent);
      
      expect(blocks[0]).toMatchObject({
        type: 'image',
        url: 'empty_alt_caption.jpeg',
        altText: '',
        caption: '',
      });

      expect(blocks[1]).toMatchObject({
        type: 'image',
        url: 'empty_caption.png',
        altText: 'alt',
        caption: '',
      });

      expect(blocks[2]).toMatchObject({
        type: 'image',
        url: 'empty_alt.webp',
        altText: '',
        caption: 'caption',
      });

      expect(blocks[3]).toMatchObject({
        type: 'image',
        url: 'full.gif',
        altText: 'alt',
        caption: 'caption',
      });

      expect(blocks[4]).toMatchObject({
        type: 'image',
        url: 'long_alt.png',
        altText: 'long alt',
        caption: '',
      });

      expect(blocks[5]).toMatchObject({
        type: 'image',
        url: 'long_caption.png',
        altText: '',
        caption: 'long caption',
      });

      expect(blocks[6]).toMatchObject({
        type: 'image',
        url: 'no_line_between.jpeg',
        altText: 'no line between',
        caption: '',
      });
    });
  });
});
