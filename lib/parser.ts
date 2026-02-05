import type { CustomBlock } from '../dist';
import type {
  Block,
  CodeBlock,
  HeadingBlock,
  ImageBlock,
  Link,
  ListBlock,
  ParagraphBlock,
  QuoteBlock,
  TextBody,
  TextBodyStyle,
  ThematicBreakBlock,
} from './definition';

type RegexpToParserPair<C extends CustomBlock<string, {}> = never> = [
  RegExp,
  (input: string) => Block<C>,
];

export class Amp<C extends CustomBlock<string, {}> = never> {
  #regexpToParserPairs: RegexpToParserPair<any>[] = [];

  constructor() {
    this.extend([paragraphRegexp, parseParagraphBlock])
      .extend([thematicBreakRegexp, parseThematicBreakBlock])
      .extend([codeRegexp, parseCodeBlock])
      .extend([imageRegexp, parseImageBlock])
      .extend([listRegexp, parseListBlock])
      .extend([quoteRegexp, parseQuoteBlock])
      .extend([headingRegexp, parseHeadingBlock]);
  }
  public extend<NewC extends CustomBlock<string, {}>>(
    regexpToParserPair: RegexpToParserPair<NewC>,
  ): Amp<C | NewC> {
    this.#regexpToParserPairs.unshift(regexpToParserPair);
    return this;
  }

  public parse(input: string) {
    const { head, body } = split(input);
    const frontmatter = parseFrontmatter(head);
    const blocks = this.#parseBlocks(body, this.#regexpToParserPairs);
    return { frontmatter, blocks };
  }

  #parseBlocks(
    input: string,
    regexpToParserPairs: RegexpToParserPair[],
  ): Block<C>[] {
    if (input.trim() === '') {
      return [];
    }

    for (const [regexp, parser] of regexpToParserPairs) {
      if (regexp.test(input)) {
        const match = regexp.exec(input);
        if (!match) {
          throw new Error('Invalid block');
        }
        return [
          parser(match[0].trim()),
          ...this.#parseBlocks(
            input.slice(match[0].length).trim(),
            regexpToParserPairs,
          ),
        ];
      }
    }

    throw new Error(`No matching block found for input: ${input}`);
  }
}

export const split = (input: string) => {
  const trimmed = input.trim();
  const match = trimmed.match(/^(---\n[\s\S]*?---)\n*/);
  const head = match?.[1] || '';
  const body = trimmed.slice(head.length).trim();

  return {
    head,
    body,
  };
};

export const parseFrontmatter = (input: string): Record<string, string> => {
  const content = input.match(/---\n+([\s\S]+)---/)?.[1];
  if (!content) {
    return {};
  }

  const lines = content.split(/\n+/).filter((line) => line.trim() !== '');
  const map = new Map();
  for (const line of lines) {
    const match = line.match(/^(.+?):\s(.+)/);
    if (!match) {
      continue;
    }
    const [_, key, value] = match;
    if (!key || !value) {
      continue;
    }
    const trimmedKey = key.trim();
    const trimmedValue = value.trim().replace(/^["']?(.+?)["']?$/, '$1');

    map.set(trimmedKey, trimmedValue);
  }

  return Object.fromEntries(map);
};

export const parseBlocks = (input: string): Block[] => {
  if (input.trim() === '') {
    return [];
  }

  for (const [regexp, parser] of regexpToParserPairs) {
    if (regexp.test(input)) {
      const match = regexp.exec(input);
      if (!match) {
        throw new Error('Invalid block');
      }
      return [
        parser(match[0].trim()),
        ...parseBlocks(input.slice(match[0].length).trim()),
      ];
    }
  }

  throw new Error(`No matching block found for input: ${input}`);
};

const headingRegexp = new RegExp(/^(#{1,6})\s(.+)/);
const quoteRegexp = new RegExp(/^(?:>\s.*\n?)+/);
const listRegexp = new RegExp(/^(?:(?:-|\d+\.)\s(.+)\n?)+/);
const imageRegexp = new RegExp(/^!\[(.*)\]\((.+?)\)(.*)/);
const codeRegexp = new RegExp(/^```(\w+)?\n([\s\S]*?)\n```/);
const thematicBreakRegexp = new RegExp(/^-{3,}/);
const linkRegexp = new RegExp(/\[.+?\]\(.+?\)/);
const paragraphRegexp = new RegExp(/^([\s\S]+?)(?:\n|$)/);

export const parseParagraphBlock = (input: string): ParagraphBlock => ({
  type: 'paragraph',
  body: parseTextBody(input),
});

export const parseTextBody = (input: string): (TextBody | Link)[] => {
  const linkParsedTextList = parseLinkInText(input);
  return linkParsedTextList
    .map((item) => (typeof item === 'string' ? parseTextBodyStyle(item) : item))
    .flat();
};

type RawStyle =
  | 'plain'
  | 'strong'
  | 'code'
  | 'asteriskItalic'
  | 'underscoreItalic';
const convertRawStyleToTextBodyStyle = (style: RawStyle): TextBodyStyle => {
  switch (style) {
    case 'asteriskItalic':
    case 'underscoreItalic':
      return 'italic';
    case 'code':
    case 'plain':
    case 'strong':
      return style;
    default:
      throw new Error(`Unknown style ${style satisfies never}`);
  }
};
const lookupUntilClose = (
  input: string,
  regularPattern: RegExp,
  unclosedPattern: RegExp | null,
  style: TextBodyStyle,
) => {
  const match = input.match(regularPattern);
  if (!match) {
    if (!unclosedPattern) {
      throw new Error(
        `Regular pattern not matched for style ${style} and unclosed pattern unknown`,
      );
    }
    const unclosedMatch = input.match(unclosedPattern);
    if (!unclosedMatch) {
      throw new Error(
        `Regular pattern and unclosed pattern not matched for style ${style}`,
      );
    }
    const [, value, rest] = unclosedMatch;
    const result = {
      type: 'textBody',
      value,
      style: 'plain',
    } satisfies TextBody;
    return { result, rest };
  }
  const [, value, rest] = match;
  const result = {
    type: 'textBody',
    value,
    style,
  } satisfies TextBody;
  return { result, rest };
};
const regularPattern = {
  asteriskItalic: /^\*{1}([^\*_`]+)\*{1}([\s\S]*)/,
  underscoreItalic: /^_{1}([^\*_`]+)_{1}([\s\S]*)/,
  code: /^`([^`]+)`([\s\S]*)/,
  strong: /^\*{2}([^\*_`]+)\*{2}([\s\S]*)/,
  plain: /^([^*_`]+)([\s\S]*)$/,
} as const satisfies Record<RawStyle, RegExp>;
const unclosedPattern = {
  asteriskItalic: /^(\*{1}[^\*_`]+?)([\*_`][\s\S]+)*$/,
  underscoreItalic: /^(_{1}[^\*_`]+?)([\*_`][\s\S]+)*$/,
  code: /^(`[^`]+)$/,
  strong: /^(\*{2}[^\*_`]+?)([\*_`][\s\S]+)*$/,
  plain: null,
} as const satisfies Record<RawStyle, RegExp | null>;

export const parseTextBodyStyle = (input: string | undefined): TextBody[] => {
  if (!input) {
    return [];
  }
  const headSymbol = checkHeadSymbol(input);
  const { result, rest } = lookupUntilClose(
    input,
    regularPattern[headSymbol],
    unclosedPattern[headSymbol],
    convertRawStyleToTextBodyStyle(headSymbol),
  );
  return [result, ...parseTextBodyStyle(rest)];
};
export const checkHeadSymbol = (input: string): RawStyle => {
  if (/^\*{2}/.test(input)) {
    return 'strong';
  }
  if (/^\*{1}/.test(input)) {
    return 'asteriskItalic';
  }
  if (/^_{1}/.test(input)) {
    return 'underscoreItalic';
  }
  if (/^`{1}/.test(input)) {
    return 'code';
  }
  return 'plain';
};

export const parseLinkInText = (input: string): (string | Link)[] => {
  const linkMatch = input.match(linkRegexp);

  if (!linkMatch) {
    return [input];
  }

  const [link] = linkMatch;
  const body = parseTextBodyStyle(link.match(/\[(.+)\]/)?.[1] ?? '');
  const url = link.match(/\((.+)\)/)?.[1] ?? '';
  const linkBlock = {
    type: 'link',
    body,
    url,
  } satisfies Link;

  const linkStartIndex = input.indexOf(link);
  const before = input.slice(0, linkStartIndex);
  const after = input.slice(linkStartIndex + link.length);

  return [before, linkBlock, ...parseLinkInText(after)].filter(Boolean);
};

export const splitLinkFromText = (input: string): string[] => {
  const linkMatch = input.match(linkRegexp);

  if (!linkMatch) {
    return [input];
  }

  const [link] = linkMatch;

  const linkStartIndex = input.indexOf(link);
  const before = input.slice(0, linkStartIndex);
  const after = input.slice(linkStartIndex + link.length);

  return [before, link, ...splitLinkFromText(after)].filter(Boolean);
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
  const match = input.match(quoteRegexp);
  if (!match) {
    throw new Error('Invalid quote block');
  }
  const text = match[0].replace(/\n>[ ]?/g, '\n').replace(/^>\s/, '');
  return {
    type: 'quote',
    body: parseTextBody(text),
  };
};

export const parseListBlock = (input: string): ListBlock => {
  const lines = input.split(/\n+/).filter((line) => line.trim() !== '');
  const matches = lines.map((line) => line.match(listRegexp));
  const ordered = matches.every((match) => match && /^\d{1,}\./.test(match[0]));
  const items = lines.map((line) => parseListItem(line));

  return {
    type: 'list',
    ordered,
    items,
  };
};

const parseListItem = (line: string): ListBlock['items'][number] => {
  const match = line.match(listRegexp);
  if (!match) {
    throw new Error('Invalid list item');
  }
  const content = match[1];
  const body = parseTextBody(content);

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

const regexpToParserPairs = [
  [headingRegexp, parseHeadingBlock],
  [quoteRegexp, parseQuoteBlock],
  [listRegexp, parseListBlock],
  [imageRegexp, parseImageBlock],
  [codeRegexp, parseCodeBlock],
  [thematicBreakRegexp, parseThematicBreakBlock],
  [paragraphRegexp, parseParagraphBlock],
] satisfies [RegExp, (input: string) => Block][];
