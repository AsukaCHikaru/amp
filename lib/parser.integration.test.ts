import { readFileSync } from 'fs';
import { join } from 'path';
import { parse } from './parser';
import { describe, expect, test } from 'bun:test';

describe('Parser Integration Test', () => {
  test('result base structure', () => {
    const markdownContent = readFileSync(
      join(process.cwd(), 'test/body.test.md'),
      'utf-8',
    );
    const result = parse(markdownContent);

    expect(result).toBeArray();
    expect(result.length).toBeGreaterThan(0);
    // The file has 11 blocks (6 headings, 2 paragraphs, 1 heading, 1 heading, 1 quote)
    expect(result.length).toBe(11);
  });
  test('parses test/body.test.md correctly', () => {
    const markdownContent = readFileSync(
      join(process.cwd(), 'test/body.test.md'),
      'utf-8',
    );
    const result = parse(markdownContent);

    expect(result[0]).toMatchObject({
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

    expect(result[1]).toMatchObject({
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

    expect(result[2]).toMatchObject({
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

    expect(result[3]).toMatchObject({
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

    expect(result[4]).toMatchObject({
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

    expect(result[5]).toMatchObject({
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

    expect(result[6]).toMatchObject({
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

    expect(result[7]).toMatchObject({
      type: 'heading',
      level: 1,
      body: [{ type: 'textBody', style: 'plain', value: 'H1' }],
    });

    expect(result[8]).toMatchObject({
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

    expect(result[9]).toMatchObject({
      type: 'heading',
      level: 2,
      body: [{ type: 'textBody', style: 'plain', value: 'H2' }],
    });

    expect(result[10]).toMatchObject({
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
          value: ' and [link](https://example.com).',
        },
      ],
    });
  });
});
