import type { TextBody, Link } from './definition';
import { parseTextBody } from './parser';
import { splitLinkFromText } from './splitLinkFromText';

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
