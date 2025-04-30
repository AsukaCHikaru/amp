import { describe, expect, it } from 'bun:test';
import { split } from './parser';

describe('split', () => {
  it('should split markdown with frontmatter and body', () => {
    const input = `---
title: Test Title
tags: tag1, tag2
---
This is the body content.

With multiple paragraphs.`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\ntitle: Test Title\ntags: tag1, tag2\n---',
      body: 'This is the body content.\n\nWith multiple paragraphs.',
    });
  });

  it('should handle markdown with only frontmatter', () => {
    const input = `---
title: Just Frontmatter
author: Test Author
---`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\ntitle: Just Frontmatter\nauthor: Test Author\n---',
      body: '',
    });
  });

  it('should handle markdown with only body (no frontmatter)', () => {
    const input = `This is just body content.

No frontmatter here.`;

    const result = split(input);

    expect(result).toEqual({
      head: '',
      body: 'This is just body content.\n\nNo frontmatter here.',
    });
  });

  it('should handle empty markdown', () => {
    const input = '';

    const result = split(input);

    expect(result).toEqual({
      head: '',
      body: '',
    });
  });

  it('should handle frontmatter with special characters', () => {
    const input = `---
title: Special * Characters & Symbols
description: This has some "quotes" and 'apostrophes'
---
Body content here.`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\ntitle: Special * Characters & Symbols\ndescription: This has some "quotes" and \'apostrophes\'\n---',
      body: 'Body content here.',
    });
  });

  it('should handle frontmatter with numbers', () => {
    const input = `---
count: 42
rating: 4.5
---
Numeric frontmatter test.`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\ncount: 42\nrating: 4.5\n---',
      body: 'Numeric frontmatter test.',
    });
  });

  it('should handle frontmatter with arrays', () => {
    const input = `---
tags: one, two, three
categories: cat1, cat2
---
Array frontmatter test.`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\ntags: one, two, three\ncategories: cat1, cat2\n---',
      body: 'Array frontmatter test.',
    });
  });

  it('should handle malformed frontmatter', () => {
    const input = `---
This is not properly formatted
frontmatter without colons
---
Body content.`;

    const result = split(input);

    expect(result).toEqual({
      head: '---\nThis is not properly formatted\nfrontmatter without colons\n---',
      body: 'Body content.',
    });
  });

  it('should handle broken frontmatter', () => {
    const input = `---
This is not properly formatted
frontmatter without colons
--
Body content.`;

    const result = split(input);

    expect(result).toEqual({
      head: '',
      body: '---\nThis is not properly formatted\nfrontmatter without colons\n--\nBody content.',
    });
  });
});
