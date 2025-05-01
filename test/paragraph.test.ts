import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/paragraph.test.md'),
  'utf-8',
);

describe('paragraph', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Paragraph test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about paragraph test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('paragraph-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      expect(blocks).toHaveLength(5);
      expect(blocks.every((block) => block.type === 'paragraph')).toBe(true);
    });

    test('block content', () => {
      const { blocks } = parse(markdownContent);

      expect(blocks[0]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            value:
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu metus egestas, eleifend tellus id, dignissim purus. Donec iaculis, dui ut pulvinar lacinia, massa magna fermentum elit, id faucibus augue orci non ex. Phasellus ultrices sem tellus, eu cursus mauris condimentum et. Morbi scelerisque sapien non erat venenatis, at volutpat sem consequat. Fusce id velit hendrerit, aliquet justo et, consectetur velit. Mauris tristique risus nunc, sit amet pulvinar felis venenatis non. Suspendisse eget ipsum fermentum, luctus est quis, lacinia lacus. Proin vulputate lectus quis porttitor tincidunt. Aenean eget ex ac justo hendrerit congue.',
          },
        ],
      });

      expect(blocks[1]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Paragraph with a line between. Cras porttitor eros nec cursus pharetra. Pellentesque ac blandit risus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Fusce nec elementum nisi. Donec efficitur lacus vel congue vehicula. Vestibulum eget sodales enim. Proin rutrum commodo erat ac lobortis. Aenean et egestas nulla, eu sodales nisl.',
          },
        ],
      });

      expect(blocks[2]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Paragraph without line between. Sed ut porttitor eros. Fusce gravida mi sed velit interdum, quis vehicula neque condimentum. Curabitur condimentum porttitor magna, sit amet dignissim ante ullamcorper in. In mattis velit sit amet orci rhoncus sodales. Vestibulum posuere accumsan cursus. Mauris nec libero tempor, tincidunt erat aliquam, rhoncus metus. Mauris cursus mattis elit, nec aliquam eros iaculis sed. Ut convallis dapibus faucibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi sit amet sodales sem, a eleifend turpis. Cras facilisis felis vitae nulla tempus facilisis. Aliquam erat volutpat. Nulla egestas sollicitudin nulla non blandit. Fusce ut sagittis nisl, quis malesuada nulla.',
          },
        ],
      });

      expect(blocks[3]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Paragraph with many lines between. Cras a iaculis velit. Cras volutpat dolor lorem, sit amet mattis odio pretium eget. Etiam id tellus nec nulla vulputate ornare vel quis neque. Suspendisse lorem dui, hendrerit in mollis porta, laoreet eget lectus. Curabitur et turpis faucibus, viverra dolor id, pharetra nisi. In ante dui, bibendum eu ex quis, dapibus facilisis tellus. Suspendisse vehicula, lorem sed vehicula condimentum, mi risus hendrerit leo, in ultricies odio tortor sed urna. Ut convallis, magna gravida bibendum ultrices, ex ex finibus leo, sit amet euismod est magna ut ex. Phasellus malesuada sapien sed sapien interdum interdum.',
          },
        ],
      });

      expect(blocks[4]).toMatchObject({
        type: 'paragraph',
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Paragraph with styled texts. Vestibulum porta dapibus mi, ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'vitae',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' tincidunt ',
          },
          {
            type: 'textBody',
            style: 'strong',
            value: 'velit dignissim varius.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'Vivamus',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' tincidunt ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'in tortor porta feugiat.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'Suspendisse',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' vitae ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'velit sit amet odio accumsan eleifend efficitur vel eros.',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'Praesent',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' sit ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'amet tincidunt mauris,',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' nec ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'dictum',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' augue. ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'Vivamus eget porttitor odio',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ', id cursus nunc. Duis vel consequat mauris. Maecenas ut nunc a lorem convallis gravida in id est.',
          },
        ],
      });
    });
  });
});
