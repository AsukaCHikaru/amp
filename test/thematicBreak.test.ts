import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/thematicBreak.test.md'),
  'utf-8',
);

describe('thematicBreak', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Thematic break test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about thematic break test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('thematic-break-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      const thematicBreakBlocks = blocks.filter(
        (block) => block.type === 'thematicBreak',
      );

      expect(thematicBreakBlocks).toHaveLength(4);
    });

    test('should parse standard thematic break', () => {
      const { blocks } = parse(markdownContent);

      const thematicBreakBlocks = blocks.filter(
        (block) => block.type === 'thematicBreak',
      ) as import('../lib/definition').ThematicBreakBlock[];

      expect(thematicBreakBlocks[0]).toMatchObject({
        type: 'thematicBreak',
      });
    });

    test('should parse thematic break with more than three hyphens', () => {
      const { blocks } = parse(markdownContent);

      const thematicBreakBlocks = blocks.filter(
        (block) => block.type === 'thematicBreak',
      ) as import('../lib/definition').ThematicBreakBlock[];

      expect(thematicBreakBlocks[1]).toMatchObject({
        type: 'thematicBreak',
      });
    });

    test('should parse consecutive thematic breaks', () => {
      const { blocks } = parse(markdownContent);

      const thematicBreakBlocks = blocks.filter(
        (block) => block.type === 'thematicBreak',
      ) as import('../lib/definition').ThematicBreakBlock[];

      expect(thematicBreakBlocks[2]).toMatchObject({
        type: 'thematicBreak',
      });
      expect(thematicBreakBlocks[3]).toMatchObject({
        type: 'thematicBreak',
      });
    });

    test('should not parse invalid thematic break with only two hyphens', () => {
      const { blocks } = parse(markdownContent);

      expect(blocks[1]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: '--',
          },
        ],
      });
    });
  });
});
