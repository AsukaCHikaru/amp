export type TextBodyStyle = 'plain' | 'italic' | 'strong' | 'code';
export type TextBody = {
  type: 'textBody';
  style: TextBodyStyle;
  value: string;
};
export type Link = {
  type: 'link';
  body: TextBody[];
  url: string;
};

export type FootnoteReference = {
  type: 'footnoteReference';
  label: string;
};

export type InlineContent = TextBody | Link | FootnoteReference;

export type ParagraphBlock = {
  type: 'paragraph';
  body: InlineContent[];
};

export type HeadingBlock = {
  type: 'heading';
  body: InlineContent[];
  level: 1 | 2 | 3 | 4 | 5 | 6;
};

export type QuoteBlock = {
  type: 'quote';
  body: InlineContent[];
};

type ListItem = {
  type: 'listItem';
  body: InlineContent[];
};

export type ListBlock = {
  type: 'list';
  items: ListItem[];
  ordered: boolean;
};

export type ImageBlock = {
  type: 'image';
  url: string;
  altText: string;
  caption: string;
};

export type CodeBlock = {
  type: 'code';
  lang?: string;
  body: string;
};

export type ThematicBreakBlock = {
  type: 'thematicBreak';
};

type FootnoteItem = {
  type: 'footnoteItem';
  label: string;
  body: InlineContent[];
};
export type FootnoteBlock = {
  type: 'footnote';
  items: FootnoteItem[];
};

export type CustomBlock<
  CustomType extends string,
  Body extends Record<string, string | number | boolean | object>,
> = {
  type: 'custom';
  customType: CustomType;
} & Body;

export type Block<TCustom extends CustomBlock<string, {}> = never> =
  | ParagraphBlock
  | HeadingBlock
  | QuoteBlock
  | ListBlock
  | ImageBlock
  | CodeBlock
  | ThematicBreakBlock
  | FootnoteBlock
  | TCustom;
