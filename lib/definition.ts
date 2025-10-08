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

export type ParagraphBlock = {
  type: 'paragraph';
  body: (TextBody | Link)[];
};

export type HeadingBlock = {
  type: 'heading';
  body: (TextBody | Link)[];
  level: 1 | 2 | 3 | 4 | 5 | 6;
};

export type QuoteBlock = {
  type: 'quote';
  body: (TextBody | Link)[];
};

type ListItem = {
  type: 'listItem';
  body: (TextBody | Link)[];
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
  | TCustom;
