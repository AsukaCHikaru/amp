import type { Block, HeadingBlock, QuoteBlock } from './definition';
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

  return { type: 'paragraph', body: parseTextBody(input) };
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
