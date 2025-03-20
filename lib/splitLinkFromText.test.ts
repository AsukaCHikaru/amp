import { describe, expect, test } from 'bun:test';
import { splitLinkFromText } from './splitLinkFromText';

describe('splitLinkFromText', () => {
  // 1. Input with no links
  test('returns array with input when no links are present', () => {
    const input = 'This is a plain text with no links';
    const expected = [input];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('returns array with input for empty string', () => {
    const input = '';
    const expected = [''];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  // 2. Input with one link
  test('extracts a single link', () => {
    const input = '[link text](https://example.com)';
    const expected = ['[link text](https://example.com)'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('extracts a single link with text before', () => {
    const input = 'Text before [link text](https://example.com)';
    const expected = ['Text before ', '[link text](https://example.com)'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('extracts a single link with text after', () => {
    const input = '[link text](https://example.com) text after';
    const expected = ['[link text](https://example.com)', ' text after'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('extracts a single link with text before and after', () => {
    const input = 'text before [link text](https://example.com) text after';
    const expected = [
      'text before ',
      '[link text](https://example.com)',
      ' text after',
    ];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  // 3. Input with multiple links
  test('extracts multiple links', () => {
    const input =
      '[first link](https://example.com) and [second link](https://example.org)';
    const expected = [
      '[first link](https://example.com)',
      ' and ',
      '[second link](https://example.org)',
    ];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('extracts multiple links with text before, between, and after', () => {
    const input =
      'start [first](https://example.com) middle [second](https://example.org) end';
    const expected = [
      'start ',
      '[first](https://example.com)',
      ' middle ',
      '[second](https://example.org)',
      ' end',
    ];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  // 4. Edge cases
  test('extracts link with empty text', () => {
    const input = '[](https://example.com)';
    const expected = ['[](https://example.com)'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('extracts link with empty URL', () => {
    const input = '[link text]()';
    const expected = ['[link text]()'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('handles links with special characters in URL', () => {
    const input = '[link text](https://example.com?param=value&another=true)';
    const expected = [
      '[link text](https://example.com?param=value&another=true)',
    ];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('handles links with special characters in text', () => {
    const input = '[link & text with special chars!](https://example.com)';
    const expected = ['[link & text with special chars!](https://example.com)'];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  // 5. Complex cases
  test('handles adjacent links', () => {
    const input =
      '[first link](https://example.com)[second link](https://example.org)';
    const expected = [
      '[first link](https://example.com)',
      '[second link](https://example.org)',
    ];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  // 6. Malformed links
  test('ignores malformed links without closing bracket', () => {
    const input = 'Text with (https://example.com[malformed link';
    const expected = [input];
    expect(splitLinkFromText(input)).toEqual(expected);
  });

  test('ignores malformed links without closing parenthesis', () => {
    const input = 'Text with (https://example.com[malformed link]';
    const expected = [input];
    expect(splitLinkFromText(input)).toEqual(expected);
  });
});
