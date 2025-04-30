import type { TextBody } from './definition';
import { parseTextBody } from './parser';
import { describe, expect, test } from 'bun:test';

describe('parseTextBody', () => {
  // 1. Parse full plain text
  test('parses full plain text', () => {
    const input = 'This is a plain text paragraph';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is a plain text paragraph',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 2. Strong text tests
  test('parses strong text only', () => {
    const input = '**strong**';
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses strong text at start', () => {
    const input = '**strong** text follows';
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text follows',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses strong text in middle', () => {
    const input = 'text with **strong** in the middle';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text with ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' in the middle',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses strong text at end', () => {
    const input = 'text ends with **strong**';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text ends with ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 2. Italic text tests (with asterisks)
  test('parses italic text only (asterisks)', () => {
    const input = '*italic*';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text at start (asterisks)', () => {
    const input = '*italic* text follows';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text follows',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text in middle (asterisks)', () => {
    const input = 'text with *italic* in the middle';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text with ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' in the middle',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text at end (asterisks)', () => {
    const input = 'text ends with *italic*';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text ends with ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 2. Italic text tests (with underscores)
  test('parses italic text only (underscores)', () => {
    const input = '_italic_';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text at start (underscores)', () => {
    const input = '_italic_ text follows';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text follows',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text in middle (underscores)', () => {
    const input = 'text with _italic_ in the middle';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text with ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' in the middle',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses italic text at end (underscores)', () => {
    const input = 'text ends with _italic_';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text ends with ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 2. Code text tests
  test('parses code text only', () => {
    const input = '`code`';
    const expected = [
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses code text at start', () => {
    const input = '`code` text follows';
    const expected = [
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text follows',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses code text in middle', () => {
    const input = 'text with `code` in the middle';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text with ',
      },
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' in the middle',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses code text at end', () => {
    const input = 'text ends with `code`';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text ends with ',
      },
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 3. Parse long styled text (more than one word)
  test('parses long strong text', () => {
    const input = '**multiple words in strong text**';
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'multiple words in strong text',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses long italic text (asterisks)', () => {
    const input = '*multiple words in italic text*';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'multiple words in italic text',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses long italic text (underscores)', () => {
    const input = '_multiple words in italic text_';
    const expected = [
      {
        type: 'textBody',
        style: 'italic',
        value: 'multiple words in italic text',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses long code text', () => {
    const input = '`const x = function() { return true; }`';
    const expected = [
      {
        type: 'textBody',
        style: 'code',
        value: 'const x = function() { return true; }',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 4. Parse mixed styled text
  test('parses text with mixed styles', () => {
    const input = 'Plain **strong** and *italic* and `code` text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Plain ',
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
        value: ' and ',
      },
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses complex mixed styles', () => {
    const input =
      '**Strong** at start, *italic* in middle, and `code` at the end';
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'Strong',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' at start, ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' in middle, and ',
      },
      {
        type: 'textBody',
        style: 'code',
        value: 'code',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' at the end',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  // 5. Link tests (parsed as plain text)
  test('parses link only as plain text', () => {
    const input = '[link text](https://example.com)';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: '[link text](https://example.com)',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses link at start as plain text', () => {
    const input = '[link text](https://example.com) followed by text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: '[link text](https://example.com) followed by text',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses link in middle as plain text', () => {
    const input = 'Text before [link text](https://example.com) and after';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Text before [link text](https://example.com) and after',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses link at end as plain text', () => {
    const input = 'Text ending with [link text](https://example.com)';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Text ending with [link text](https://example.com)',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses multiple links as plain text', () => {
    const input =
      '[first link](https://example.com) and [second link](https://example.org)';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value:
          '[first link](https://example.com) and [second link](https://example.org)',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });

  test('parses links with styled text as plain text', () => {
    const input =
      'Text with **strong** and [link text](https://example.com) and *italic*';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Text with ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' and [link text](https://example.com) and ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(parseTextBody(input)).toEqual(expected);
  });
});
