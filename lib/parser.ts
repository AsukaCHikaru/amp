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

export const parse = (input: string) => {
  const { head, body } = split(input);
  const frontmatter = parseFrontmatter(head);
  const lines = body.split(/\n{2,}?/).filter((line) => line.trim() !== '');
  const blocks = lines.map(parseBlock);
  return { frontmatter, blocks };
};

export const split = (input: string) => {
  const match = input.match(/(---\n+(.*\n+)+---)\n*/);
  const head = match?.[1] || '';
  const body = input.slice(head.length).trim();

  return {
    head,
    body,
  };
};

export const parseFrontmatter = (
  input: string,
): Record<string, string | string[] | number> => {
  const content = input.match(/---\n+((.+\n+)+)---/)?.[1];
  if (!content) {
    return {};
  }

  const lines = content.split(/\n+/).filter((line) => line.trim() !== '');
  const map = new Map();
  for (const line of lines) {
    const [key, value] = line.split(/:\s+/);
    if (!key || !value) {
      continue;
    }
    const trimmedKey = key.trim();
    const trimmedValue = value.trim();

    if (trimmedValue.includes(',')) {
      map.set(
        trimmedKey,
        trimmedValue.split(',').map((v) => v.trim()),
      );
      continue;
    }

    if (!isNaN(Number(trimmedValue))) {
      map.set(trimmedKey, Number(trimmedValue));
      continue;
    }

    map.set(trimmedKey, trimmedValue);
  }

  return Object.fromEntries(map);
};

const headingRegexp = new RegExp(/^(#{1,6})\s+(.+)$/);
const quoteRegexp = new RegExp(/^>\s+(.+)$/);
const listRegexp = new RegExp(/^(-|\d{1,}\.)\s+(.+)$/);
const imageRegexp = new RegExp(/^!\[(.*)\]\((.+?)\)(.*)$/);
const codeRegexp = new RegExp(/^```(\w+)?\n([\s\S]*?)\n```$/);
const thematicBreakRegexp = new RegExp(/^-{3,}$/);
const linkRegexp = new RegExp(/\[.+?\]\(.+?\)/);

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

export const parseParagraphBlock = (input: string): ParagraphBlock => ({
  type: 'paragraph',
  body: parseTextBody(input)
    .map((textBody) =>
      textBody.style === 'plain' && linkRegexp.test(textBody.value)
        ? parseLinkInTextBody(textBody)
        : textBody,
    )
    .flat(),
});

export const parseTextBody = (input: string): TextBody[] => {
  const { result } = parseTextBodyStyle({
    text: input,
    progress: null,
    result: [],
  });

  return result.map(({ style, text }) => ({
    type: 'textBody',
    style,
    value: text,
  }));
};

type TextBodyParseResult = {
  text: string;
  progress: { style: TextBodyStyle; text: string } | null;
  result: { style: TextBodyStyle; text: string }[];
};
export const parseTextBodyStyle = ({
  text,
  progress,
  result,
}: TextBodyParseResult): TextBodyParseResult => {
  if (text.length === 0) {
    return {
      text,
      result: progress ? [...result, progress] : result,
      progress: null,
    };
  }

  const { style: headStyle, text: headText } = checkHeadStyle(text);
  const progressStyle = progress?.style;
  const first = headText[0];
  const rest = headText.slice(1);

  switch (progressStyle) {
    case undefined:
      return parseTextBodyStyle({
        text: rest,
        progress: { style: headStyle, text: first },
        result,
      });
    case 'plain':
      switch (headStyle) {
        case 'plain':
          return parseTextBodyStyle({
            text: rest,
            progress: { style: 'plain', text: (progress?.text ?? '') + first },
            result,
          });
        case 'code':
        case 'italic':
        case 'strong':
          return parseTextBodyStyle({
            text: rest,
            progress: { style: headStyle, text: first },
            result: progress ? [...result, progress] : result,
          });
        default:
          throw new Error(`Unexpected style: ${headStyle satisfies never}`);
      }
    case 'code':
    case 'italic':
    case 'strong':
      if (headStyle === progressStyle) {
        return parseTextBodyStyle({
          text: headText,
          progress: null,
          result: [
            ...result,
            { style: progressStyle, text: progress?.text ?? '' },
          ],
        });
      }
      return parseTextBodyStyle({
        text: rest,
        progress: {
          style: progressStyle,
          text: (progress?.text ?? '') + first,
        },
        result,
      });
    default:
      throw new Error(`Unexpected style: ${progressStyle satisfies never}`);
  }
};

const checkHeadStyle = (
  text: string,
): {
  style: TextBodyStyle;
  text: string;
} => {
  if (/^\*{2}/.test(text)) {
    return { style: 'strong', text: text.slice(2) };
  }
  if (/^\*{1}/.test(text)) {
    return { style: 'italic', text: text.slice(1) };
  }
  if (/^_{1}/.test(text)) {
    return { style: 'italic', text: text.slice(1) };
  }
  if (/^`{1}/.test(text)) {
    return { style: 'code', text: text.slice(1) };
  }
  return { style: 'plain', text };
};

export const parseLinkInTextBody = (input: TextBody): (TextBody | Link)[] => {
  const splittedText = splitLinkFromText(input.value);
  return splittedText.map((text) => {
    if (/\[.+\]\(.+\)/.test(text)) {
      const linkText = text.match(/\[(.+)\]/)?.[1] ?? '';
      const url = text.match(/\((.+)\)/)?.[1] ?? '';
      return {
        type: 'link',
        body: parseTextBody(linkText),
        url,
      };
    }
    return {
      type: 'textBody',
      style: input.style,
      value: text,
    };
  });
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
  const match = quoteRegexp.exec(input);
  if (!match) {
    throw new Error('Invalid quote block');
  }
  const text = match[1];
  return {
    type: 'quote',
    body: parseTextBody(text)
      .map((textBody) =>
        textBody.style === 'plain' && linkRegexp.test(textBody.value)
          ? parseLinkInTextBody(textBody)
          : textBody,
      )
      .flat(),
  };
};

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
    .map((textBody) =>
      textBody.style === 'plain' && linkRegexp.test(textBody.value)
        ? parseLinkInTextBody(textBody)
        : textBody,
    )
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
