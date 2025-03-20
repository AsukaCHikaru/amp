const linkRegex = /\[.+?\]\(.+?\)/;

export const splitLinkFromText = (input: string): string[] => {
  const linkMatch = input.match(linkRegex);

  if (!linkMatch) {
    return [input];
  }

  const [link] = linkMatch;

  const linkStartIndex = input.indexOf(link);
  const before = input.slice(0, linkStartIndex);
  const after = input.slice(linkStartIndex + link.length);

  return [before, link, ...splitLinkFromText(after)].filter(Boolean);
};
