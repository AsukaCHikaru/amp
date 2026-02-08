import type { TextBody } from '../dist';
import { mergeSameTypeTextBody } from '../lib/parser';
import { describe, expect, test } from 'vitest';

describe('mergeSameTypeTextBody', () => {
  // 1. Empty input
  test('returns empty array for empty input', () => {
    const input = [] satisfies TextBody[];
    const expected = [] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 2. Single TextBody element
  test('returns single TextBody element unchanged', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 3. No adjacent same-style TextBody
  test('returns array unchanged when no adjacent TextBody share the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'bold',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' text',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'italic',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 4. Two consecutive same-style TextBody
  test('merges two consecutive TextBody with the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: ' world',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'hello world',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 5. Three or more consecutive same-style
  test('merges three or more consecutive TextBody with the same style', () => {
    const input = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'one',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'two',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'three',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'strong',
        value: 'onetwothree',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });

  // 6. Multiple groups of same-style merged independently
  test('merges multiple groups of same-style TextBody independently', () => {
    const input = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'a',
      },
      {
        type: 'textBody',
        style: 'plain',
        value: 'b',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'c',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'd',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'e',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'f',
      },
    ] satisfies TextBody[];
    const expected = [
      {
        type: 'textBody',
        style: 'plain',
        value: 'ab',
      },
      {
        type: 'textBody',
        style: 'strong',
        value: 'cd',
      },
      {
        type: 'textBody',
        style: 'italic',
        value: 'ef',
      },
    ] satisfies TextBody[];
    expect(mergeSameTypeTextBody(input)).toEqual(expected);
  });
});
