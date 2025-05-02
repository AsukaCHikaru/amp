import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/code.test.md'),
  'utf-8',
);

describe('code', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Code test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about code test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('code-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      // Filter only code blocks
      const codeBlocks = blocks.filter((block) => block.type === 'code');

      // We expect 5 code blocks to be parsed correctly
      expect(codeBlocks).toHaveLength(5);
    });

    test('should parse single line code with language', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access code properties
      const codeBlocks = blocks.filter(
        (block) => block.type === 'code',
      ) as import('../lib/definition').CodeBlock[];

      // First block: single line code with js language
      expect(codeBlocks[0]).toMatchObject({
        type: 'code',
        lang: 'js',
        body: '// single line code',
      });
    });

    test('should parse code without language specification', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access code properties
      const codeBlocks = blocks.filter(
        (block) => block.type === 'code',
      ) as import('../lib/definition').CodeBlock[];

      // Second block: no language specified
      expect(codeBlocks[1]).toMatchObject({
        type: 'code',
        lang: undefined,
        body: 'no lang',
      });
    });

    test('should parse multi-line code with language', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access code properties
      const codeBlocks = blocks.filter(
        (block) => block.type === 'code',
      ) as import('../lib/definition').CodeBlock[];

      // Third block: multiple lines with js language
      expect(codeBlocks[2]).toMatchObject({
        type: 'code',
        lang: 'js',
        body: '// multiple lang code\n\nconst x = 5;\n\nconsole.log(x);',
      });
    });

    test('should parse code with other languages', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access code properties
      const codeBlocks = blocks.filter(
        (block) => block.type === 'code',
      ) as import('../lib/definition').CodeBlock[];

      // Fourth block: python language
      expect(codeBlocks[3]).toMatchObject({
        type: 'code',
        lang: 'python',
        body: '# other lang',
      });
    });

    test('should parse code blocks with no line between them', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access code properties
      const codeBlocks = blocks.filter(
        (block) => block.type === 'code',
      ) as import('../lib/definition').CodeBlock[];

      // Fifth block: typescript with no line between
      expect(codeBlocks[4]).toMatchObject({
        type: 'code',
        lang: 'ts',
        body: '// no line between',
      });
    });
  });
});
