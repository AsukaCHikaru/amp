import { readFileSync } from 'fs';
import { describe, expect, test } from 'vitest';
import { join } from 'path';
import { Amp, parseTextBody } from '../lib/parser';
import type { FootnoteBlock, InlineContent } from '../lib/definition';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/footnote.test.md'),
  'utf-8',
);

describe('footnote', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const amp = new Amp();
      const { frontmatter } = amp.parse(markdownContent);

      expect(frontmatter['title']).toBe('Footnote test');
      expect(frontmatter['pathname']).toBe('footnote-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('references', () => {
    test('parses a reference inside a heading', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[0]).toEqual({
        type: 'heading',
        level: 1,
        body: [
          { type: 'textBody', style: 'plain', value: 'Heading with footnote' },
          { type: 'footnoteReference', label: '1' },
        ],
      });
    });

    test('parses a reference in the middle of a paragraph', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[1]).toEqual({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Paragraph with a footnote',
          },
          { type: 'footnoteReference', label: '2' },
          { type: 'textBody', style: 'plain', value: ' in the middle.' },
        ],
      });
    });

    test('parses a reference at the end of a paragraph', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[2]).toEqual({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Paragraph ending with a footnote',
          },
          { type: 'footnoteReference', label: '3' },
        ],
      });
    });

    test('parses multiple references in one paragraph', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[3]).toEqual({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Paragraph with two footnotes',
          },
          { type: 'footnoteReference', label: '4' },
          { type: 'textBody', style: 'plain', value: ' and another' },
          { type: 'footnoteReference', label: '5' },
          { type: 'textBody', style: 'plain', value: ' here.' },
        ],
      });
    });

    test('parses a reference inside a quote', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[4]).toEqual({
        type: 'quote',
        body: [
          { type: 'textBody', style: 'plain', value: 'Quote with footnote' },
          { type: 'footnoteReference', label: '6' },
        ],
      });
    });

    test('parses a reference inside a list item', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[5]).toEqual({
        type: 'list',
        ordered: false,
        items: [
          {
            type: 'listItem',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'List item with footnote',
              },
              { type: 'footnoteReference', label: '7' },
            ],
          },
        ],
      });
    });

    test('parses a reference alongside a link without either eating the other', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      expect(blocks[6]).toEqual({
        type: 'paragraph',
        body: [
          { type: 'textBody', style: 'plain', value: 'Text with ' },
          {
            type: 'link',
            body: [{ type: 'textBody', style: 'plain', value: 'a link' }],
            url: 'https://example.com',
          },
          { type: 'textBody', style: 'plain', value: ' and a footnote' },
          { type: 'footnoteReference', label: '8' },
          { type: 'textBody', style: 'plain', value: ' together.' },
        ],
      });
    });

    test('leaves a reference with no definition in place', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      const definedLabels = blocks
        .filter((block): block is FootnoteBlock => block.type === 'footnote')
        .flatMap((block) => block.items)
        .map((item) => item.label);

      expect(definedLabels).not.toContain('8');
    });
  });

  describe('definitions', () => {
    test('groups adjacent definition lines into a single block', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);

      const footnoteBlocks = blocks.filter(
        (block): block is FootnoteBlock => block.type === 'footnote',
      );

      expect(footnoteBlocks).toHaveLength(1);
      expect(footnoteBlocks[0].items.map((item) => item.label)).toEqual([
        '1',
        '2',
        '3',
        '9',
      ]);
    });

    test('parses a plain definition body', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);
      const block = blocks[7] as FootnoteBlock;

      expect(block.items[0]).toEqual({
        type: 'footnoteItem',
        label: '1',
        body: [{ type: 'textBody', style: 'plain', value: 'First note.' }],
      });
    });

    test('parses a definition body with styled text', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);
      const block = blocks[7] as FootnoteBlock;

      expect(block.items[1]).toEqual({
        type: 'footnoteItem',
        label: '2',
        body: [
          { type: 'textBody', style: 'plain', value: 'Note with ' },
          { type: 'textBody', style: 'strong', value: 'strong' },
          { type: 'textBody', style: 'plain', value: ' text.' },
        ],
      });
    });

    test('parses a definition body with a link', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);
      const block = blocks[7] as FootnoteBlock;

      expect(block.items[2]).toEqual({
        type: 'footnoteItem',
        label: '3',
        body: [
          { type: 'textBody', style: 'plain', value: 'Note with ' },
          {
            type: 'link',
            body: [{ type: 'textBody', style: 'plain', value: 'a link' }],
            url: 'https://example.com',
          },
          { type: 'textBody', style: 'plain', value: '.' },
        ],
      });
    });

    test('keeps a definition that is never referenced', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(markdownContent);
      const block = blocks[7] as FootnoteBlock;

      expect(block.items[3]).toEqual({
        type: 'footnoteItem',
        label: '9',
        body: [
          { type: 'textBody', style: 'plain', value: 'Never referenced.' },
        ],
      });
    });

    test('parses a definition written before its reference', () => {
      const amp = new Amp();
      const { blocks } = amp.parse(
        '[^1]: The note.\n\nThe paragraph[^1] comes second.',
      );

      expect(blocks).toEqual([
        {
          type: 'footnote',
          items: [
            {
              type: 'footnoteItem',
              label: '1',
              body: [{ type: 'textBody', style: 'plain', value: 'The note.' }],
            },
          ],
        },
        {
          type: 'paragraph',
          body: [
            { type: 'textBody', style: 'plain', value: 'The paragraph' },
            { type: 'footnoteReference', label: '1' },
            { type: 'textBody', style: 'plain', value: ' comes second.' },
          ],
        },
      ]);
    });

    test('starts a new block when definitions are separated by a blank line', () => {
      const amp = new Amp();
      const { blocks } = amp.parse('[^1]: One.\n\n[^2]: Two.');

      expect(blocks).toHaveLength(2);
      expect(blocks.map((block) => (block as FootnoteBlock).items)).toEqual([
        [
          {
            type: 'footnoteItem',
            label: '1',
            body: [{ type: 'textBody', style: 'plain', value: 'One.' }],
          },
        ],
        [
          {
            type: 'footnoteItem',
            label: '2',
            body: [{ type: 'textBody', style: 'plain', value: 'Two.' }],
          },
        ],
      ]);
    });

    test('parses a definition with no space after the colon', () => {
      const amp = new Amp();
      const { blocks } = amp.parse('[^1]:The note.');
      const block = blocks[0] as FootnoteBlock;

      expect(block.items[0]).toEqual({
        type: 'footnoteItem',
        label: '1',
        body: [{ type: 'textBody', style: 'plain', value: 'The note.' }],
      });
    });

    test('parses a definition with an empty body', () => {
      const amp = new Amp();
      const { blocks } = amp.parse('[^1]:');
      const block = blocks[0] as FootnoteBlock;

      expect(block.items[0]).toEqual({
        type: 'footnoteItem',
        label: '1',
        body: [],
      });
    });
  });

  describe('parseTextBody', () => {
    test('parses a reference on its own', () => {
      const expected = [
        { type: 'footnoteReference', label: '1' },
      ] satisfies InlineContent[];
      expect(parseTextBody('[^1]')).toEqual(expected);
    });

    test('parses a reference adjacent to styled text', () => {
      const expected = [
        { type: 'textBody', style: 'strong', value: 'strong' },
        { type: 'footnoteReference', label: '12' },
        { type: 'textBody', style: 'italic', value: 'italic' },
      ] satisfies InlineContent[];
      expect(parseTextBody('**strong**[^12]*italic*')).toEqual(expected);
    });

    test('does not treat a bracketed caret without a closing bracket as a reference', () => {
      const expected = [
        { type: 'textBody', style: 'plain', value: 'not [^1 a footnote' },
      ] satisfies InlineContent[];
      expect(parseTextBody('not [^1 a footnote')).toEqual(expected);
    });
  });
});
