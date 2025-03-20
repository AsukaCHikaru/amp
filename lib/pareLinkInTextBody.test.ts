import type { TextBody, Link } from "./definition";
import { parseLinkInTextBody } from "./parseLinkInTextBody";
import { describe, expect, test } from 'bun:test';

describe('parseLinkInTextBody', () => {
  // 1. No links in TextBody
  test('returns original TextBody when no links are present', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: 'This is a plain text with no links',
    };
    const expected = [input];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 2. TextBody with only a link
  test('parses TextBody with only a link', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[link text](https://example.com)',
    };
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'link text',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 3. TextBody with text before a link
  test('parses TextBody with text before a link', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: 'Text before [link text](https://example.com)',
    };
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Text before ',
      },
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'link text',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 4. TextBody with text after a link
  test('parses TextBody with text after a link', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[link text](https://example.com) text after',
    };
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'link text',
          },
        ],
        url: 'https://example.com',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text after',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 5. TextBody with text before and after a link
  test('parses TextBody with text before and after a link', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: 'text1 and [link](https://example.com) and text2',
    };
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'text1 and ',
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
        value: ' and text2',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 6. TextBody with multiple links
  test('parses TextBody with multiple links', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[first](https://example.com) and [second](https://example.org)',
    };
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'first',
          },
        ],
        url: 'https://example.com',
      },
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
            value: 'second',
          },
        ],
        url: 'https://example.org',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 7. TextBody with styled text and links
  test('parses TextBody with styled text and links', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: 'Text with **strong** and [link](https://example.com)',
    };
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'Text with **strong** and ',
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
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 8. TextBody with a link that has styled text
  test('parses TextBody with a link that has styled text markers (treated as plain text)', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[**bold link**](https://example.com)',
    };
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'strong',
            value: 'bold link',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 9. Edge case: TextBody with a link that has empty text
  test('parses TextBody with a link that has empty text', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[](https://example.com)',
    };
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: '[](https://example.com)',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 10. Edge case: TextBody with a link that has empty URL
  test('parses TextBody with a link that has empty URL', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[link text]()',
    };
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: '[link text]()',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 11. Edge case: TextBody with non-plain style
  test('preserves TextBody style when no links are present', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'strong',
      value: 'This is strong text with no links',
    };
    const expected = [input];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });

  // 12. Edge case: TextBody with a link that has special characters
  test('parses TextBody with a link that has special characters', () => {
    const input: TextBody = {
      type: 'textBody',
      style: 'plain',
      value: '[link & text!](https://example.com?param=value&another=true)',
    };
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'link & text!',
          },
        ],
        url: 'https://example.com?param=value&another=true',
      },
    ] satisfies (TextBody | Link)[];
    expect(parseLinkInTextBody(input)).toEqual(expected);
  });
});
