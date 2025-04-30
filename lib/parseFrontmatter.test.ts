import { describe, expect, it } from 'bun:test';
import { parseFrontmatter } from './parser';

describe('parseFrontmatter', () => {
  it('should parse frontmatter with string values', () => {
    const input = `---
title: Hello World
description: This is a test
author: John Doe
---

# Content starts here`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      title: 'Hello World',
      description: 'This is a test',
      author: 'John Doe',
    });
  });

  it('should parse frontmatter with number values', () => {
    const input = `---
version: 1
count: 42
rating: 4.5
---

Some content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      version: '1',
      count: '42',
      rating: '4.5',
    });
  });

  it('should parse frontmatter with string array values', () => {
    const input = `---
tags: javascript, typescript, react
categories: programming, web
---

Content here`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      tags: 'javascript, typescript, react',
      categories: 'programming, web',
    });
  });

  it('should parse frontmatter with number values in arrays', () => {
    const input = `---
scores: 98, 87, 92
ratings: 4.5, 3.8, 5.0
---

Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      scores: '98, 87, 92',
      ratings: '4.5, 3.8, 5.0',
    });
  });

  it('should parse frontmatter with mixed value types', () => {
    const input = `---
title: Mixed Types Example
version: 2
tags: test, example
counts: 1, 2, 3
---

# Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      title: 'Mixed Types Example',
      version: '2',
      tags: 'test, example',
      counts: '1, 2, 3',
    });
  });

  it('should return empty object for empty frontmatter', () => {
    const input = `---
---

Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({});
  });

  it('should return empty object when no frontmatter is present', () => {
    const input = `# No frontmatter here
    
Just regular content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({});
  });

  it('should handle frontmatter with whitespace', () => {
    const input = `---
  title:    Spaced Content   
  tags:     one ,  two  
---

Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      title: 'Spaced Content',
      tags: 'one ,  two',
    });
  });

  it('should ignore content after frontmatter', () => {
    const input = `---
title: Just the frontmatter
---

# This should be ignored
And this too`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({
      title: 'Just the frontmatter',
    });
  });

  it('should handle malformed frontmatter by returning empty object', () => {
    const input = `--
title: Missing a dash
--

Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({});
  });

  it('should handle frontmatter without closing delimiter', () => {
    const input = `---
title: No closing
author: Someone

Content`;

    const result = parseFrontmatter(input);

    expect(result).toEqual({});
  });
});
