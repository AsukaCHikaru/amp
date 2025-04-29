import type {
  Block,
  CodeBlock,
  HeadingBlock,
  ImageBlock,
  ListBlock,
  ParagraphBlock,
  QuoteBlock,
  ThematicBreakBlock,
} from './definition';
import { parseLinkInTextBody } from './parseLinkInTextBody';
import { parseTextBody } from './parseTextBody';

export const parse = (input: string) => {
  const lines = input.split(/\n{2,}?/).filter((line) => line.trim() !== '');
  const blocks = lines.map(parseBlock);
  return blocks;
};

const headingRegexp = new RegExp(/^(#{1,6})\s+(.+)$/);
const quoteRegexp = new RegExp(/^>\s+(.+)$/);
const listRegexp = new RegExp(/^(-|\d{1,}\.)\s+(.+)$/);
const imageRegexp = new RegExp(/^!\[(.*)\]\((.+?)\)(.*)$/);
const codeRegexp = new RegExp(/^```(\w+)?\n([\s\S]*?)\n```$/);
const thematicBreakRegexp = new RegExp(/^-{3,}$/);

export const parseBlock = (input: string): Block => {
  if (headingRegexp.test(input)) {
    return parseHeadingBlock(input);
  }
  if (quoteRegexp.test(input)) {
    return parseQuoteBlock(input);
  }
  if (input.split(/\n+/).every((line) => listRegexp.test(line))) {
    return parseListBlock(input);
  }
  if (imageRegexp.test(input)) {
    return parseImageBlock(input);
  }
  if (codeRegexp.test(input)) {
    return parseCodeBlock(input);
  }
  if (thematicBreakRegexp.test(input)) {
    return parseThematicBreakBlock();
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
  const lines = input.split(/\n+/).filter((line) => line.trim() !== '');
  const matches = lines.map((line) => listRegexp.exec(line));
  const ordered = matches.every((match) => match && /^\d{1,}\./.test(match[1]));
  const items = lines.map((line) => parseListItem(line));

  return {
    type: 'list',
    ordered,
    items,
  };
};

const parseListItem = (line: string): ListBlock['items'][number] => {
  const match = listRegexp.exec(line);
  if (!match) {
    throw new Error('Invalid list item');
  }
  const content = match[2];
  const body = parseTextBody(content)
    .map((textBody) => {
      return textBody.style === 'plain' && /\[.+\]\(.+\)/.test(textBody.value)
        ? parseLinkInTextBody(textBody)
        : textBody;
    })
    .flat();
  return {
    type: 'listItem',
    body,
  };
};

export const parseImageBlock = (input: string): ImageBlock => {
  const match = imageRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid image block');
  }
  const altText = match[1];
  const url = match[2];
  const caption = match[3].replace(/^\((.+)\)$/, '$1') || '';

  return {
    type: 'image',
    url,
    altText,
    caption,
  };
};

export const parseCodeBlock = (input: string): CodeBlock => {
  const match = codeRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid code block');
  }
  const lang = match[1] || undefined;
  const body = match[2];

  return {
    type: 'code',
    lang,
    body,
  };
};

export const parseThematicBreakBlock = (): ThematicBreakBlock => ({
  type: 'thematicBreak',
});
