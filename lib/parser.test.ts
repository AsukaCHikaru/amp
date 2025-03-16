import type { TextBody } from './definition';
import { parseTextBodyStyle } from './parser';
import { describe, expect, test } from 'bun:test';

describe('parseTextBodyStyle', () => {
  test('parses plain text paragraph', () => {
    const input = 'This is a plain text paragraph';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is a plain text paragraph',
      },
    ] satisfies TextBody[];
    expect(parseTextBodyStyle(input)).toEqual(expected);
  });

  test('parses paragraph with strong text', () => {
    const input = 'This is **strong** text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'strong',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
    ] satisfies TextBody[];
    expect(parseTextBodyStyle(input)).toEqual(expected);
  });

  test('parses paragraph with italic text using asterisks', () => {
    const input = 'This is *italic* text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
    ] satisfies TextBody[];
    expect(parseTextBodyStyle(input)).toEqual(expected);
  });

  test('parses paragraph with italic text using underscores', () => {
    const input = 'This is _italic_ text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is ',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
    ] satisfies TextBody[];
    expect(parseTextBodyStyle(input)).toEqual(expected);
  });

  test('parses paragraph with code', () => {
    const input = 'This is `code` text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is ',
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

  test('parses paragraph with mixed styles', () => {
    const input =
      'This is **strong** and *italic* and _also italic_ and `code` text';
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'This is ',
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
        style: 'italic',
        value: 'also italic',
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
});
