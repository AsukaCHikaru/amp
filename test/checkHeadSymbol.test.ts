import { checkHeadSymbol } from '../lib/parser';
import { describe, expect, test } from 'vitest';

describe('checkHeadSymbol', () => {
  // strong: starts with ** followed by non-* character
  describe('strong', () => {
    test('returns strong for **text', () => {
      expect(checkHeadSymbol('**text')).toBe('strong');
    });

    test('returns strong for ** followed by a space', () => {
      expect(checkHeadSymbol('** ')).toBe('strong');
    });

    test('returns strong for **a (minimum case)', () => {
      expect(checkHeadSymbol('**a')).toBe('strong');
    });
  });

  // asteriskItalic: starts with single * followed by non-* character
  describe('asteriskItalic', () => {
    test('returns asteriskItalic for *text', () => {
      expect(checkHeadSymbol('*text')).toBe('asteriskItalic');
    });

    test('returns asteriskItalic for * followed by a space', () => {
      expect(checkHeadSymbol('* ')).toBe('asteriskItalic');
    });

    test('returns asteriskItalic for *a (minimum case)', () => {
      expect(checkHeadSymbol('*a')).toBe('asteriskItalic');
    });
  });

  // underscoreItalic: starts with single _ followed by non-_ character
  describe('underscoreItalic', () => {
    test('returns underscoreItalic for _text', () => {
      expect(checkHeadSymbol('_text')).toBe('underscoreItalic');
    });

    test('returns underscoreItalic for _ followed by a space', () => {
      expect(checkHeadSymbol('_ ')).toBe('underscoreItalic');
    });

    test('returns underscoreItalic for _a (minimum case)', () => {
      expect(checkHeadSymbol('_a')).toBe('underscoreItalic');
    });
  });

  // code: starts with single ` followed by non-` character
  describe('code', () => {
    test('returns code for `text', () => {
      expect(checkHeadSymbol('`text')).toBe('code');
    });

    test('returns code for ` followed by a space', () => {
      expect(checkHeadSymbol('` ')).toBe('code');
    });

    test('returns code for `a (minimum case)', () => {
      expect(checkHeadSymbol('`a')).toBe('code');
    });
  });

  // plain: no special leading marker
  describe('plain', () => {
    test('returns plain for regular text', () => {
      expect(checkHeadSymbol('hello')).toBe('plain');
    });

    test('returns plain for text starting with a number', () => {
      expect(checkHeadSymbol('123')).toBe('plain');
    });

    test('returns plain for text starting with a space', () => {
      expect(checkHeadSymbol(' text')).toBe('plain');
    });

    test('returns plain for empty string', () => {
      expect(checkHeadSymbol('')).toBe('plain');
    });
  });

  // Priority: ** is checked before * (strong takes precedence over asteriskItalic)
  describe('priority', () => {
    test('** is detected as strong, not asteriskItalic', () => {
      expect(checkHeadSymbol('**bold**')).toBe('strong');
    });

    test('***x returns strong (** matched first)', () => {
      expect(checkHeadSymbol('***x')).toBe('strong');
    });

    test('****x returns strong (** matched first)', () => {
      expect(checkHeadSymbol('****x')).toBe('strong');
    });
  });

  // Edge cases: markers with special characters after them
  describe('edge cases - special characters after markers', () => {
    test('returns strong for ** followed by underscore', () => {
      expect(checkHeadSymbol('**_')).toBe('strong');
    });

    test('returns strong for ** followed by backtick', () => {
      expect(checkHeadSymbol('**`')).toBe('strong');
    });

    test('returns asteriskItalic for * followed by underscore', () => {
      expect(checkHeadSymbol('*_')).toBe('asteriskItalic');
    });

    test('returns asteriskItalic for * followed by backtick', () => {
      expect(checkHeadSymbol('*`')).toBe('asteriskItalic');
    });

    test('returns underscoreItalic for _ followed by asterisk', () => {
      expect(checkHeadSymbol('_*')).toBe('underscoreItalic');
    });

    test('returns underscoreItalic for _ followed by backtick', () => {
      expect(checkHeadSymbol('_`')).toBe('underscoreItalic');
    });

    test('returns code for ` followed by asterisk', () => {
      expect(checkHeadSymbol('`*')).toBe('code');
    });

    test('returns code for ` followed by underscore', () => {
      expect(checkHeadSymbol('`_')).toBe('code');
    });
  });
});
