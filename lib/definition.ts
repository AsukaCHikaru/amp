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
  body: TextBody[];
  level: 1 | 2 | 3 | 4 | 5 | 6;
};

export type QuoteBlock = {
  type: 'quote';
  body: TextBody[];
};

export type Block = ParagraphBlock | HeadingBlock | QuoteBlock;
