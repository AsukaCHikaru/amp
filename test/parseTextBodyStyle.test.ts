import type { TextBody } from '../dist';
import { parseTextBodyStyle } from '../lib/parser';
import { describe, expect, test } from 'vitest';

describe('parseTextBodyStyleV2', () => {
  // Empty input
  test('returns empty array for empty string', () => {
    expect(parseTextBodyStyle('')).toEqual([]);
  });

  describe('Plain', () => {
    test('plain', () => {
      const input = 'plaintext';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'plaintext',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
    test('plain with space', () => {
      const input = 'plain text';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'plain text',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Strong', () => {
    test('strong only', () => {
      const input = '**strong**';
      const expected = [
        {
          type: 'textBody',
          style: 'strong',
          value: 'strong',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('strong at start', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('strong in middle', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('strong at end', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('strong with multiple words', () => {
      const input = '**multiple words in strong**';
      const expected = [
        {
          type: 'textBody',
          style: 'strong',
          value: 'multiple words in strong',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Italic (asterisks)', () => {
    test('italic only', () => {
      const input = '*italic*';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic at start', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic in middle', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic at end', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic with multiple words', () => {
      const input = '*multiple words in italic*';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'multiple words in italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Italic (underscores)', () => {
    test('italic only', () => {
      const input = '_italic_';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic at start', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic in middle', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic at end', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('italic with multiple words', () => {
      const input = '_multiple words in italic_';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'multiple words in italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Code', () => {
    test('code only', () => {
      const input = '`code`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'code',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code at start', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code in middle', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code at end', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with multiple words', () => {
      const input = '`const x = function() { return true; }`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'const x = function() { return true; }',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Inline code edge cases', () => {
    test('code with underscores inside', () => {
      const input = '`text_with_underscore`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'text_with_underscore',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with asterisks inside', () => {
      const input = '`text*with*asterisks`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'text*with*asterisks',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with double asterisks inside (not bold)', () => {
      const input = '`**not bold**`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: '**not bold**',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with underscores inside (not italic)', () => {
      const input = '`_not italic_`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: '_not italic_',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with HTML tags', () => {
      const input = '`<div>`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: '<div>',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with single space', () => {
      const input = '` `';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: ' ',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with multiple spaces', () => {
      const input = '`code with   spaces`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'code with   spaces',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Mixed styles', () => {
    test('all styles mixed', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('styled at start, middle, and end', () => {
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
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code between italic (underscores)', () => {
      const input = '_italic_ `code` _italic_';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' ',
        },
        {
          type: 'textBody',
          style: 'code',
          value: 'code',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' ',
        },
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code between strong', () => {
      const input = '**bold** `code` **bold**';
      const expected = [
        {
          type: 'textBody',
          style: 'strong',
          value: 'bold',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' ',
        },
        {
          type: 'textBody',
          style: 'code',
          value: 'code',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'bold',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code adjacent to italic without spaces', () => {
      const input = '*italic*`code`*italic*';
      const expected = [
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
        {
          type: 'textBody',
          style: 'code',
          value: 'code',
        },
        {
          type: 'textBody',
          style: 'italic',
          value: 'italic',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('multiple code spans with plain text and italic', () => {
      const input = '`a` and `b` with _c_';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: 'a',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' and ',
        },
        {
          type: 'textBody',
          style: 'code',
          value: 'b',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' with ',
        },
        {
          type: 'textBody',
          style: 'italic',
          value: 'c',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with underscores surrounded by italic', () => {
      const input = 'Use _emphasis_ with `snake_case_variable` in _context_';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'Use ',
        },
        {
          type: 'textBody',
          style: 'italic',
          value: 'emphasis',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' with ',
        },
        {
          type: 'textBody',
          style: 'code',
          value: 'snake_case_variable',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' in ',
        },
        {
          type: 'textBody',
          style: 'italic',
          value: 'context',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('code with mixed markdown syntax inside', () => {
      const input = '`**bold** and _italic_ and [link](url)`';
      const expected = [
        {
          type: 'textBody',
          style: 'code',
          value: '**bold** and _italic_ and [link](url)',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });

  describe('Unclosed markers', () => {
    test('unclosed strong is cut at end', () => {
      const input = 'text **unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'text ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '**unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed italic (asterisk) is cut at end', () => {
      const input = 'text *unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'text ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '*unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed italic (underscore) is cut at end', () => {
      const input = 'text _unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'text ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '_unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed code is cut at end', () => {
      const input = 'text `unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: 'text ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '`unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed marker at the very start', () => {
      const input = '**unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '**unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed strong cut at next unclosed italic', () => {
      const input = '**unclosed _also unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '**unclosed ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '_also unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed italic cut at next unclosed code', () => {
      const input = '*unclosed `also unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '*unclosed ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '`also unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('three different unclosed markers each cut at next', () => {
      const input = '_one **two `three';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '_one ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '**two ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '`three',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('closed style followed by different unclosed marker', () => {
      const input = '**bold** then _unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'strong',
          value: 'bold',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' then ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '_unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('different unclosed marker followed by closed style', () => {
      const input = '_unclosed **bold**';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '_unclosed ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'bold',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('unclosed underscore cut at next unclosed strong', () => {
      const input = '_unclosed **strong';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '_unclosed ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '**strong',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });

    test('closed style between two unclosed markers', () => {
      const input = '_unclosed **bold** `also unclosed';
      const expected = [
        {
          type: 'textBody',
          style: 'plain',
          value: '_unclosed ',
        },
        {
          type: 'textBody',
          style: 'strong',
          value: 'bold',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: ' ',
        },
        {
          type: 'textBody',
          style: 'plain',
          value: '`also unclosed',
        },
      ] satisfies TextBody[];
      expect(parseTextBodyStyle(input)).toEqual(expected);
    });
  });
});
