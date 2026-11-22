import { toPlainInfix, toLatex } from './latex';
import { describe, expect, test } from 'vitest';

describe("toPlainInfix", () => {

  test('converts LaTeX expressions to plain infix', () => {
    expect(toPlainInfix('x^2')).toBe('x^2');
    expect(toPlainInfix('\\frac{1}{2}')).toBe('(1)/(2)');
    expect(toPlainInfix('sinc')).toBe('sinc');
    expect(toPlainInfix('b a')).toBe('b*a');
  });
})

describe("toLatex", () => {
  test('converts plain expressions to LaTeX', () => {
    expect(toLatex('x^2')).toBe('x^{2}');
    expect(toLatex('sin(x)')).toBe('\\sin\\left(x\\right)');
    expect(toLatex('sinx(x)')).toBe('\\operatorname{sinc}\\left(x\\right)');
  });
})
