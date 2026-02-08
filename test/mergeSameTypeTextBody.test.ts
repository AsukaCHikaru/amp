import type { Link, TextBody } from '../dist';
import { mergeSameTypeTextBody } from '../lib/parser';
import { describe, expect, test } from 'vitest';

describe('mergeSameTypeTextBody', () => {
  // 1. Empty input
  test('returns empty array for empty input', () => {
    const input = [] satisfies (TextBody | Link)[];
    const expected = [] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 2. Single TextBody element
  test('returns single TextBody element unchanged', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 3. Single Link element
  test('returns single Link element unchanged', () => {
    const input = [
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
    ] satisfies Link[];
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
    ] satisfies Link[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 4. No adjacent same-style TextBody
  test('returns array unchanged when no adjacent TextBody share the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 5. Two consecutive same-style TextBody
  test('merges two consecutive TextBody with the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' world',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello world',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 6. Three or more consecutive same-style
  test('merges three or more consecutive TextBody with the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'one',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'two',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'three',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'onetwothree',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 7. Multiple groups of same-style merged independently
  test('merges multiple groups of same-style TextBody independently', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'a',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: 'b',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'c',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'd',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'e',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'f',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'ab',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'cd',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'ef',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 8. Link between two same-style TextBody prevents merging
  test('does not merge same-style TextBody across a Link', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'before',
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
        value: 'after',
      },
    ] satisfies (TextBody | Link)[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'before',
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
        value: 'after',
      },
    ] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 9. Multiple Links in a row preserved separately
  test('preserves multiple consecutive Links separately', () => {
    const input = [
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
    ] satisfies Link[];
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
    ] satisfies Link[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 10. Link body with consecutive same-style TextBody gets merged
  test('merges consecutive same-style TextBody inside Link body', () => {
    const input = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'hello',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' world',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'hello world',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 11. Link body with different-style TextBody stays unchanged
  test('does not merge different-style TextBody inside Link body', () => {
    const input = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'strong',
            value: 'bold',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' text',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    const expected = [
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'strong',
            value: 'bold',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' text',
          },
        ],
        url: 'https://example.com',
      },
    ] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 12. Link body merge combined with top-level merge
  test('merges both Link body TextBody and top-level TextBody', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'before',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' link',
      },
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'italic',
            value: 'a',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'b',
          },
        ],
        url: 'https://example.com',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' after',
      },
    ] satisfies (TextBody | Link)[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'before link',
      },
      {
        type: 'link',
        body: [
          {
            type: 'textBody',
            style: 'italic',
            value: 'ab',
          },
        ],
        url: 'https://example.com',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' after',
      },
    ] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 13. Mixed TextBody and Link complex scenario
  test('handles mixed TextBody and Link in a complex scenario', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'start ',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: 'continue ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
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
        style: 'italic',
        value: 'ital',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'ic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' end',
      },
    ] satisfies (TextBody | Link)[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'start continue ',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
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
        style: 'italic',
        value: 'italic',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' end',
      },
    ] satisfies (TextBody | Link)[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });
});
