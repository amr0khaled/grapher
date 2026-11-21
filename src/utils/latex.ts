export function toPlainInfix(latex: string): string {
  let out = latex;
  // Convert LaTeX superscripts ^{exp} back to ^exp
  out = out.replace(/\^\{([^}]+)\}/g, '^$1');
  // Convert fractions \frac{a}{b} back to (a)/(b)
  out = out.replace(/\\frac\{([^}]+)\}\{([^}]+)\}/g, '($1)/($2)');
  // Convert cdot to *
  out = out.replace(/\\cdot/g, '*');
  // Remove curly braces from functions
  out = out.replace(/\\(sqrt|sum|int)\{([^}]+)\}/g, '$1($2)');
  // Cleanup infinity
  out = out.replace(/\\infty/g, 'infinity');

  // Insert implicit multiplication between adjacent single letters (ba -> b*a)
  // Be careful not to break function names like sin, cos, tan
  out = out.replace(/\b([a-zA-Z])\s+([a-zA-Z])\b/g, (match, p1, p2) => {
    const combined = p1 + p2;
    if (['sin', 'cos', 'tan', 'ln', 'log', 'sqrt'].includes(combined)) {
      return match;
    }
    return `${p1}*${p2}`;
  });

  return out;
}

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

// Helper for multi-argument functions like sum(expr, a, b) / int(expr, a, b)
function replaceMultiArgFunc(text: string, name: string, latex: (expr: string, a: string, b: string) => string): string {
  let out = text;
  const lower = () => out.toLowerCase();
  while (true) {
    const idx = lower().indexOf(name + '(');
    if (idx === -1) break;
    const open = idx + name.length;
    const close = matchParen(out, open);
    if (close === -1) break;
    const inner = out.slice(open + 1, close);
    // split inner by top-level comma
    let depth = 0;
    let commaIndices = [];
    for (let i = 0; i < inner.length; i++) {
      if (inner[i] === '(') depth++;
      else if (inner[i] === ')') depth--;
      else if (inner[i] === ',' && depth === 0) commaIndices.push(i);
    }
    if (commaIndices.length === 2) {
      const expr = inner.slice(0, commaIndices[0]).trim();
      const a = inner.slice(commaIndices[0] + 1, commaIndices[1]).trim();
      const b = inner.slice(commaIndices[1] + 1).trim();
      out = out.slice(0, idx) + latex(expr, a, b) + out.slice(close + 1);
    } else {
      // fallback to single arg if not 3 args
      out = out.slice(0, idx) + name + '{' + inner + '}' + out.slice(close + 1);
    }
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

// Replace caret '^' with superscript
function replaceSuperscripts(text: string): string {
  let out = text;
  while (true) {
    const idx = out.indexOf('^');
    if (idx === -1) break;
    // left base: single alphanumeric/token or parenthesized group
    let leftStart = idx - 1;
    if (leftStart < 0) break;
    if (out[leftStart] === ')') {
      let depth = 1;
      leftStart--;
      while (leftStart >= 0 && depth > 0) {
        if (out[leftStart] === ')') depth++;
        else if (out[leftStart] === '(') depth--;
        leftStart--;
      }
      leftStart++;
    } else {
      while (leftStart >= 0 && /[\w.]/.test(out[leftStart])) leftStart--;
      leftStart++;
    }

    // right exponent: single token or parenthesized group
    let rightEnd = idx + 1;
    if (rightEnd >= out.length) break;
    if (out[rightEnd] === '(') {
      const close = matchParen(out, rightEnd);
      if (close === -1) break;
      const exp = out.slice(rightEnd + 1, close);
      const base = out.slice(leftStart, idx);
      out = out.slice(0, leftStart) + base + '^{' + exp + '}' + out.slice(close + 1);
    } else {
      while (rightEnd < out.length && /[\w.]/.test(out[rightEnd])) rightEnd++;
      const exp = out.slice(idx + 1, rightEnd);
      const base = out.slice(leftStart, idx);
      out = out.slice(0, leftStart) + base + '^{' + exp + '}' + out.slice(rightEnd);
    }
  }
  return out;
}

// Replace matrices/vectors [1, 2; 3, 4] -> \begin{bmatrix}... \end{bmatrix}
function replaceMatrices(text: string): string {
  let out = text;
  let depth = 0;
  let openIdx = -1;
  for (let i = 0; i < out.length; i++) {
    if (out[i] === '(' || out[i] === '{') depth++;
    else if (out[i] === ')' || out[i] === '}') depth--;
    else if (out[i] === '[' && depth === 0) {
      openIdx = i;
      break;
    }
  }
  if (openIdx === -1) return out;

  // find matching close bracket at depth 0
  let closeIdx = -1;
  depth = 0;
  for (let i = openIdx; i < out.length; i++) {
    if (out[i] === '[') depth++;
    else if (out[i] === ']') {
      depth--;
      if (depth === 0) {
        closeIdx = i;
        break;
      }
    }
  }
  if (closeIdx === -1) return out;

  const content = out.slice(openIdx + 1, closeIdx);
  // split rows by ';'
  const rows = content.split(';');
  const formattedRows = rows.map(row => {
    // split cols by ','
    const cols = row.split(',').map(c => c.trim()).join(' & ');
    return cols;
  }).join(' \\\\ ');

  const bmatrix = '\\begin{bmatrix}' + formattedRows + '\\end{bmatrix}';
  return replaceMatrices(out.slice(0, openIdx) + bmatrix + out.slice(closeIdx + 1));
}

export function toLatex(input: string): string {
  let out = input.trim();
  if (!out) return '';

  // Derivative notation d/dx, d/dy
  out = out.replace(/\bd\s*\/\s*dx\b/gi, '\\frac{d}{dx}');
  out = out.replace(/\bd\s*\/\s*dy\b/gi, '\\frac{d}{dy}');

  out = replaceMultiArgFunc(out, 'sum', (expr, a, b) => `\\sum_{${a}}^{${b}}{${expr}}`);
  out = replaceMultiArgFunc(out, 'int', (expr, a, b) => `\\int_{${a}}^{${b}}{${expr}}`);

  out = replaceFunc(out, 'sqrt', (a) => '\\sqrt{' + replaceFractions(a) + '}');
  out = replaceFunc(out, 'sum', (a) => '\\sum{' + replaceFractions(a) + '}');
  out = replaceFunc(out, 'int', (a) => '\\int{' + replaceFractions(a) + '}');

  out = replaceMatrices(out);
  out = replaceSuperscripts(out);

  out = out.replace(/\binfinity\b/gi, '\\infty');
  out = out.replace(/\*/g, '\\cdot ');
  out = replaceFractions(out);

  return out;
}
