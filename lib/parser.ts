import type {
  Block,
  HeadingBlock,
  ListBlock,
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

const headingRegexp = new RegExp(/^(#{1,6})\s+(.+)$/);
const quoteRegexp = new RegExp(/^>\s+(.+)$/);
const listRegexp = new RegExp(/^(-|\d{1,}\.)\s+(.+)$/);

export const parseBlock = (input: string): Block => {
  if (headingRegexp.test(input)) {
    return parseHeadingBlock(input);
  }
  if (quoteRegexp.test(input)) {
    return parseQuoteBlock(input);
  }
  if (listRegexp.test(input)) {
    return parseListBlock(input);
  }

  return parseParagraphBlock(input);
};

export const parseHeadingBlock = (input: string): HeadingBlock => {
  const match = headingRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid heading block');
  }
  const level = match[1].length as HeadingBlock['level'];
  const text = match[2];
  return {
    type: 'heading',
    level,
    body: parseTextBody(text),
  };
};

export const parseQuoteBlock = (input: string): QuoteBlock => {
  const match = quoteRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid quote block');
  }
  const text = match[1];
  return {
    type: 'quote',
    body: parseTextBody(text),
  };
};

export const parseParagraphBlock = (input: string): ParagraphBlock => ({
  type: 'paragraph',
  body: parseTextBody(input)
    .map((textBody) => {
      return textBody.style === 'plain' && /\[.+\]\(.+\)/.test(textBody.value)
        ? parseLinkInTextBody(textBody)
        : textBody;
    })
    .flat(),
});

export const parseListBlock = (input: string): ListBlock => {
  const match = listRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid list block');
  }
  const ordered = match[1]?.match(/^\d{1,}\.$/) !== null;
  const content = match[2] ?? '';
  const body = parseTextBody(content)
  .map((textBody) => {
    return textBody.style === 'plain' && /\[.+\]\(.+\)/.test(textBody.value)
      ? parseLinkInTextBody(textBody)
      : textBody;
  })
  .flat();
  
  return {
    type: 'list',
    ordered,
    body,
  }
};
