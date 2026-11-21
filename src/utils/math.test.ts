import { getEvaluatedValue } from './math';
import { expect, test } from 'vitest';

test('evaluates determinable variables', () => {
  expect(getEvaluatedValue('a = 5', {})).toBe(5);
  expect(getEvaluatedValue('b = a', { a: 10 })).toBe(10);
  expect(getEvaluatedValue('c = d', { a: 10 })).toBeNull();
});
