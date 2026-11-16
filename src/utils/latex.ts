// Convert plain-math input to LaTeX for KaTeX rendering.

// Find matching closing index for text[open] (text[open] === '(')
function matchParen(text: string, open: number): number {
  let depth = 0;
  for (let i = open; i < text.length; i++) {
    if (text[i] === '(') depth++;
    else if (text[i] === ')') {
      depth--;
      if (depth === 0) return i;
    }
  }
  return -1;
}

function replaceFunc(text: string, name: string, latex: (arg: string) => string): string {
  let out = text;
  const lower = () => out.toLowerCase();
  while (true) {
    const idx = lower().indexOf(name + '(');
    if (idx === -1) break;
    const open = idx + name.length;
    const close = matchParen(out, open);
    if (close === -1) break;
    const arg = out.slice(open + 1, close);
    out = out.slice(0, idx) + latex(arg) + out.slice(close + 1);
  }
  return out;
}

// Split top-level '/' (paren-depth 0) into [left, right] repeatedly into \frac
function replaceFractions(text: string): string {
  // find top-level '/'
  let depth = 0;
  let fracIdx = -1;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === '(' || c === '{') depth++;
    else if (c === ')' || c === '}') depth--;
    else if (c === '/' && depth === 0) { fracIdx = i; break; }
  }
  if (fracIdx === -1) return text;

  // left operand: scan back from '/'
  let leftStart = fracIdx - 1;
  // token: run of word chars, number, or balanced parenthesized group
  if (text[leftStart] === ')') {
    depth = 1;
    leftStart--;
    while (leftStart >= 0 && depth > 0) {
      if (text[leftStart] === ')') depth++;
      else if (text[leftStart] === '(') depth--;
      leftStart--;
    }
    leftStart++;
  } else {
    while (leftStart >= 0 && /[\w.]/.test(text[leftStart])) leftStart--;
    leftStart++;
  }

  // right operand
  let rightEnd = fracIdx + 1;
  if (text[rightEnd] === '(') {
    const close = matchParen(text, rightEnd);
    if (close === -1) return text;
    rightEnd = close + 1;
  } else {
    while (rightEnd < text.length && /[\w.]/.test(text[rightEnd])) rightEnd++;
  }

  const left = text.slice(leftStart, fracIdx).trim();
  const right = text.slice(fracIdx + 1, rightEnd).trim();
  const inner = left && right
    ? '\\frac{' + replaceFractions(left) + '}{' + replaceFractions(right) + '}'
    : text;
  return replaceFractions(text.slice(0, leftStart) + inner + text.slice(rightEnd));
}

export function toLatex(input: string): string {
  let out = input.trim();
  if (!out) return '';

  out = replaceFunc(out, 'sqrt', (a) => '\\sqrt{' + replaceFractions(a) + '}');
  out = replaceFunc(out, 'sum', (a) => '\\sum{' + replaceFractions(a) + '}');
  out = replaceFunc(out, 'int', (a) => '\\int{' + replaceFractions(a) + '}');

  out = out.replace(/\binfinity\b/gi, '\\infty');
  out = out.replace(/\*/g, '\\cdot ');
  out = replaceFractions(out);

  return out;
}
