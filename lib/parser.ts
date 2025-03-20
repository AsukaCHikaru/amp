import type {
  Block,
  HeadingBlock,
  ParagraphBlock,
  QuoteBlock,
} from './definition';
import { parseLinkInTextBody } from './parseLinkInTextBody';
import { parseTextBody } from './parseTextBody';

export const parse = (input: string) => {
  const lines = input.split(/\n+/).filter((line) => line.trim() !== '');
  const blocks = lines.map(parseBlock);
  return blocks;
};

export const parseBlock = (input: string): Block => {
  if (/^#{1,6}\s+.+/.test(input)) {
    return parseHeadingBlock(input);
  }
  if (/^>\s+.+/.test(input)) {
    return parseQuoteBlock(input);
  }

  return parseParagraphBlock(input);
};

export const parseHeadingBlock = (input: string): HeadingBlock => {
  const level = (input.match(/^#{1,6}/)?.[0].length ??
    1) as HeadingBlock['level'];
  const text = input.replace(/^#{1,6}\s+/, '');
  return {
    type: 'heading',
    level,
    body: parseTextBody(text),
  };
};

export const parseQuoteBlock = (input: string): QuoteBlock => {
  const text = input.replace(/^>\s+/, '');
  return {
    type: 'quote',
    body: parseTextBody(text),
  };
};
export const parseParagraphBlock = (input: string): ParagraphBlock => ({
  type: 'paragraph',
  body: parseTextBody(input)
    .map((textBody) => {
      // console.log(textBody)
      return textBody.style === 'plain' && /\[.+\]\(.+\)/.test(textBody.value)
        ? parseLinkInTextBody(textBody)
        : textBody;
    })
    .flat(),
});
