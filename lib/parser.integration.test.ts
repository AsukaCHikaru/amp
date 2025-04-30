import { readFileSync } from 'fs';
import { join } from 'path';
import { parse } from './parser';
import { describe, expect, test } from 'bun:test';

describe('Parser Integration Test', () => {
  describe('body', () => {
    test('result base structure', () => {
      const markdownContent = readFileSync(
        join(process.cwd(), 'test/body.test.md'),
        'utf-8',
      );
      const { blocks } = parse(markdownContent);

      expect(blocks).toBeArray();
      expect(blocks.length).toBeGreaterThan(0);
      // The file has 11 blocks (6 headings, 2 paragraphs, 1 heading, 1 heading, 1 quote)
      expect(blocks.length).toBe(11);
    });
    test('parses test/body.test.md correctly', () => {
      const markdownContent = readFileSync(
        join(process.cwd(), 'test/body.test.md'),
        'utf-8',
      );
      const { blocks } = parse(markdownContent);

      expect(blocks[0]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [
          { type: 'textBody', style: 'plain', value: 'H1 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[1]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [
          { type: 'textBody', style: 'plain', value: 'H2 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[2]).toMatchObject({
        type: 'heading',
        level: 3,
        body: [
          { type: 'textBody', style: 'plain', value: 'H3 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[3]).toMatchObject({
        type: 'heading',
        level: 4,
        body: [
          { type: 'textBody', style: 'plain', value: 'H4 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[4]).toMatchObject({
        type: 'heading',
        level: 5,
        body: [
          { type: 'textBody', style: 'plain', value: 'H5 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[5]).toMatchObject({
        type: 'heading',
        level: 6,
        body: [
          { type: 'textBody', style: 'plain', value: 'H6 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[6]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'This is a fully plain paragraph. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas pharetra sem in ante varius, id mollis sapien varius. Nunc et rutrum arcu.',
          },
        ],
      });

      expect(blocks[7]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [{ type: 'textBody', style: 'plain', value: 'H1' }],
      });

      expect(blocks[8]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is a paragraph with ',
          },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'strong', value: 'long strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'code' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'long code' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' and ',
          },
          {
            type: 'link',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'link',
              },
            ],
            url: 'https://example.com',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '.',
          },
        ],
      });

      expect(blocks[9]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [{ type: 'textBody', style: 'plain', value: 'H2' }],
      });

      expect(blocks[10]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is quote with ',
          },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'strong', value: 'long strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'code' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'long code' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' and ',
          },
          {
            type: 'link',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'link',
              },
            ],
            url: 'https://example.com',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '.',
          },
        ],
      });
    });
  });

  describe('full', () => {
    test('parse frontmatter', () => {
      const markdownContent = readFileSync(
        join(process.cwd(), 'test/full.test.md'),
        'utf-8',
      );
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter).toBeObject();
      expect(frontmatter['title']).toBe('Full test');
      expect(frontmatter['description']).toBe('This is, as you can see, a markdown file for the purpose of, no other than testing, which are listed in the file of `full.test.ts`.');
      expect(frontmatter['fibonacci']).toBe('1, 1, 2, 3, 5, 8, 13, 21');
    })

    test('result base structure', () => {
      const markdownContent = readFileSync(
        join(process.cwd(), 'test/body.test.md'),
        'utf-8',
      );
      const { blocks } = parse(markdownContent);

      expect(blocks).toBeArray();
      expect(blocks.length).toBeGreaterThan(0);
      // The file has 11 blocks (6 headings, 2 paragraphs, 1 heading, 1 heading, 1 quote)
      expect(blocks.length).toBe(11);
    });
    test('parses test/body.test.md correctly', () => {
      const markdownContent = readFileSync(
        join(process.cwd(), 'test/body.test.md'),
        'utf-8',
      );
      const { blocks } = parse(markdownContent);

      expect(blocks[0]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [
          { type: 'textBody', style: 'plain', value: 'H1 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[1]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [
          { type: 'textBody', style: 'plain', value: 'H2 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[2]).toMatchObject({
        type: 'heading',
        level: 3,
        body: [
          { type: 'textBody', style: 'plain', value: 'H3 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[3]).toMatchObject({
        type: 'heading',
        level: 4,
        body: [
          { type: 'textBody', style: 'plain', value: 'H4 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[4]).toMatchObject({
        type: 'heading',
        level: 5,
        body: [
          { type: 'textBody', style: 'plain', value: 'H5 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[5]).toMatchObject({
        type: 'heading',
        level: 6,
        body: [
          { type: 'textBody', style: 'plain', value: 'H6 plain ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' [link](https://example.com)',
          },
          { type: 'textBody', style: 'code', value: 'inline-code' },
        ],
      });

      expect(blocks[6]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'This is a fully plain paragraph. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas pharetra sem in ante varius, id mollis sapien varius. Nunc et rutrum arcu.',
          },
        ],
      });

      expect(blocks[7]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [{ type: 'textBody', style: 'plain', value: 'H1' }],
      });

      expect(blocks[8]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is a paragraph with ',
          },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'strong', value: 'long strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'code' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'long code' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' and ',
          },
          {
            type: 'link',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'link',
              },
            ],
            url: 'https://example.com',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '.',
          },
        ],
      });

      expect(blocks[9]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [{ type: 'textBody', style: 'plain', value: 'H2' }],
      });

      expect(blocks[10]).toMatchObject({
        type: 'quote',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'This is quote with ',
          },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'strong', value: 'long strong' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'italic', value: 'long italic' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'code' },
          { type: 'textBody', style: 'plain', value: ' and ' },
          { type: 'textBody', style: 'code', value: 'long code' },
          {
            type: 'textBody',
            style: 'plain',
            value: ' and ',
          },
          {
            type: 'link',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'link',
              },
            ],
            url: 'https://example.com',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: '.',
          },
        ],
      });
    });
  });
});
