import { readFileSync } from 'fs';
import { describe, expect, test } from 'bun:test';
import { join } from 'path';
import { parse } from '../lib/parser';

const markdownContent = readFileSync(
  join(process.cwd(), 'test/heading.test.md'),
  'utf-8',
);

describe('heading', () => {
  describe('frontmatter', () => {
    test('should parse frontmatter', () => {
      const { frontmatter } = parse(markdownContent);

      expect(frontmatter['title']).toBe('Heading test');
      expect(frontmatter['description']).toBe(
        'Sentence of the description, which is about heading test. This property exist for the purpose of, no other than frontmatter testing.',
      );
      expect(frontmatter['date']).toBe('2020-01-01');
      expect(frontmatter['datetime']).toBe('2020-01-01 12:00');
      expect(frontmatter['pathname']).toBe('heading-test');
      expect(frontmatter['category']).toBe('test');
    });
  });

  describe('blocks', () => {
    test('should parse blocks', () => {
      const { blocks } = parse(markdownContent);

      expect(blocks).toHaveLength(9);
      expect(blocks.every((block) => block.type === 'heading')).toBe(true);
    });

    test('should parse heading levels correctly', () => {
      const { blocks } = parse(markdownContent);

      // Type assertion for blocks to access level property
      const headingBlocks =
        blocks as import('../lib/definition').HeadingBlock[];

      expect(headingBlocks[0].level).toBe(1);
      expect(headingBlocks[1].level).toBe(2);
      expect(headingBlocks[2].level).toBe(3);
      expect(headingBlocks[3].level).toBe(4);
      expect(headingBlocks[4].level).toBe(5);
      expect(headingBlocks[5].level).toBe(6);
      expect(headingBlocks[6].level).toBe(1);
      expect(headingBlocks[7].level).toBe(3);
      expect(headingBlocks[8].level).toBe(2);
    });

    test('block content', () => {
      const { blocks } = parse(markdownContent);

      // H1 heading
      expect(blocks[0]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. In sed purus vel nunc tempus posuere. Nulla nulla elit, convallis vitae vulputate vel, semper id justo. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec vulputate tempor risus nec gravida. Pellentesque malesuada mauris tellus, eu tincidunt neque luctus eget. Vestibulum nisl est, gravida et condimentum et, dignissim quis sapien. Integer quis nunc id augue varius ultricies. Fusce eleifend felis tellus, interdum tristique sem egestas vitae. Donec bibendum massa quis dolor vehicula malesuada. Morbi porttitor sit amet neque vitae hendrerit. Aliquam sem felis, dictum ac dapibus vel, dictum vitae sapien.',
          },
        ],
      });

      // H2 heading
      expect(blocks[1]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec tellus ex. Mauris sapien lorem, accumsan ac efficitur in, luctus efficitur augue. In volutpat fermentum orci in porta. Aliquam mattis elementum nunc, volutpat laoreet nisi semper in. Proin cursus nisl elit. Proin at semper tellus. Aenean non commodo justo. Vivamus cursus imperdiet ipsum, eget sollicitudin nibh laoreet id. Curabitur pharetra dapibus enim quis viverra. Sed venenatis nibh nunc, vitae vulputate velit interdum eu. Sed sed lectus ex.',
          },
        ],
      });

      // H3 heading
      expect(blocks[2]).toMatchObject({
        type: 'heading',
        level: 3,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Integer quis nunc sodales, cursus mauris nec, dignissim purus. Nulla id mauris arcu. Phasellus et dui euismod, egestas nibh quis, molestie quam. Donec et metus volutpat, luctus massa ac, tempus velit. Vivamus quis consectetur odio. Ut egestas libero semper efficitur commodo. Integer fermentum odio a vulputate maximus. Nulla at maximus purus.',
          },
        ],
      });

      // H4 heading
      expect(blocks[3]).toMatchObject({
        type: 'heading',
        level: 4,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Donec nec arcu ligula. Morbi ultrices nibh quis turpis elementum, quis facilisis quam varius. Maecenas tempus ullamcorper lobortis. Sed varius tristique nibh. Pellentesque gravida id magna scelerisque ultricies. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin ullamcorper massa id nisl posuere maximus. Curabitur ornare hendrerit vulputate. Quisque ultricies eget quam et consectetur. Vestibulum scelerisque dignissim hendrerit. Donec neque ex, pulvinar vel eleifend eu, venenatis vel massa. Quisque vestibulum aliquet justo vitae sagittis. Pellentesque tellus odio, semper in consequat sagittis, bibendum ut mauris.',
          },
        ],
      });

      // H5 heading
      expect(blocks[4]).toMatchObject({
        type: 'heading',
        level: 5,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Quisque sit amet convallis urna. Etiam egestas laoreet eros at malesuada. Vivamus interdum molestie mi, sagittis consequat dolor pellentesque vitae. Quisque magna turpis, blandit sit amet risus at, lobortis cursus ante. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Proin nulla ipsum, vehicula sit amet lacus eget, posuere dapibus nunc. Aenean quis sagittis turpis, porta gravida magna. Phasellus eget ante vulputate, semper velit quis, faucibus tellus. Nunc id metus pellentesque, porta metus eget, malesuada ex. Sed mattis quam eget diam sodales, non pretium mauris semper. Nulla maximus porttitor enim maximus porta.',
          },
        ],
      });

      // H6 heading
      expect(blocks[5]).toMatchObject({
        type: 'heading',
        level: 6,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Vestibulum facilisis orci risus, at eleifend diam efficitur a. Proin sit amet ex id nisl ultrices dapibus. Phasellus elementum ipsum vitae ipsum suscipit, a consequat nisi ultricies. Suspendisse scelerisque justo eu pretium auctor. Nulla accumsan eros sit amet lorem convallis, eget tincidunt enim eleifend. Integer vel ultricies mauris, at dignissim nisl. Quisque consequat venenatis felis, a molestie lacus sollicitudin a. Curabitur ex elit, ultrices porta nisi at, porttitor lacinia turpis. Ut suscipit gravida mollis. Donec eu commodo eros, a bibendum orci. Phasellus in rutrum ante, ut tristique urna. Donec ultrices, quam at mollis tincidunt, neque quam interdum ipsum, vel dictum nisl elit pharetra nisi. Vestibulum finibus urna elit, a molestie nulla interdum posuere.',
          },
        ],
      });

      // Another H1 heading
      expect(blocks[6]).toMatchObject({
        type: 'heading',
        level: 1,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Fusce eu suscipit est, nec commodo magna. Phasellus vel imperdiet leo. Donec euismod tempus erat, et porttitor quam finibus eget. Pellentesque maximus fermentum nunc ut fermentum. Praesent eu convallis nunc. Duis mattis, sem nec rhoncus tincidunt, felis ante maximus orci, a luctus urna dui id est. Cras bibendum congue metus, sit amet vestibulum lectus laoreet vel. Aliquam fermentum fermentum eros, eget lacinia nulla imperdiet id.',
          },
        ],
      });

      // Another H3 heading
      expect(blocks[7]).toMatchObject({
        type: 'heading',
        level: 3,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value:
              'Sed in nulla vel diam consectetur imperdiet sit amet ut massa. Nam turpis nulla, fringilla in tortor id, laoreet sollicitudin tellus. Quisque rhoncus tincidunt dui sit amet semper. Donec lacinia, velit nec pretium pharetra, augue ante bibendum neque, quis scelerisque augue enim sed lectus. Quisque blandit dui sed aliquet varius. Aliquam convallis tortor et diam posuere, ac facilisis ante elementum. Quisque ac dolor nibh. Sed id felis nulla. Curabitur neque justo, eleifend et sodales eget, lacinia ut velit. Fusce sit amet justo sed nunc facilisis fermentum. In volutpat aliquet magna a finibus.',
          },
        ],
      });

      // H2 heading with styled text and links
      expect(blocks[8]).toMatchObject({
        type: 'heading',
        level: 2,
        body: [
          {
            type: 'textBody',
            style: 'plain',
            value: 'Sed felis metus, ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'sagittis',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' in ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'suscipit at,',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' consectetur ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'gravida',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' magna. ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'Orci varius natoque',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' penatibus ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'et',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' magnis dis ',
          },
          {
            type: 'textBody',
            style: 'italic',
            value: 'parturient montes, nascetur',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' ridiculus mus. ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'Cras',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' odio enim, ',
          },
          {
            type: 'link',
            url: 'https://example.com',
            body: [
              {
                type: 'textBody',
                style: 'plain',
                value: 'congue vel erat vel,',
              },
            ],
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' maximus ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'gravida',
          },
          {
            type: 'textBody',
            style: 'plain',
            value: ' tellus. ',
          },
          {
            type: 'textBody',
            style: 'code',
            value: 'Phasellus facilisis mauris libero',
          },
          {
            type: 'textBody',
            style: 'plain',
            value:
              ', sit amet hendrerit mi consequat vel. Suspendisse vulputate purus pharetra, posuere dolor non, dapibus magna. Maecenas et lacinia enim, sagittis convallis lacus. Vivamus imperdiet viverra mauris a vulputate. Suspendisse sollicitudin, augue blandit ultricies semper, turpis odio consectetur ipsum, non tincidunt erat massa id elit. Nam interdum blandit faucibus.',
          },
        ],
      });
    });
  });
});
