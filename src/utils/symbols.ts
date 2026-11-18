// Symbol table: m = 5, n = m/2 (reactive, top-down evaluation)

export interface SymbolTable {
  [name: string]: number;
}

export interface Assignment {
  name: string;
  rhs: string;
}

// "m = 5" → { name: "m", rhs: "5" } else null
export function parseAssignment(line: string): Assignment | null {
  const m = line.match(/^\s*([a-zA-Z_]\w*)\s*=\s*(.+)\s*$/);
  if (!m) return null;
  const name = m[1].toLowerCase();
  if (name === 'x' || name === 'y') return null;
  if (line.includes('(') && line.indexOf('(') < line.indexOf('=')) return null;
  return { name: m[1], rhs: m[2] };
}

// Evaluate arithmetic expression with given symbol table.
// Supports + - * / ^ ( ) and identifiers; juxtaposition = multiplication.
// Unknown identifiers throw.
export function evalExpr(expr: string, symbols: SymbolTable): number {
  const clean = expr.trim();
  if (!clean) return NaN;

  const tokens = clean.match(/(\d+\.?\d*|[a-zA-Z_]\w*|[+\-*/()^])/g) || [];
  if (tokens.join('').replace(/\s/g, '') !== clean.replace(/\s/g, '')) {
    throw new Error('bad characters');
  }

  // Insert implicit multiplication:
  //   number/symbol followed by number/symbol/'('
  //   ')' followed by number/symbol/'('
  const out: string[] = [];
  for (let i = 0; i < tokens.length; i++) {
    const cur = tokens[i];
    const prev = tokens[i - 1];
    if (prev) {
      const prevEnds = /^\d|^[a-zA-Z_]/.test(prev) || prev === ')';
      const curStarts = /^\d|^[a-zA-Z_]/.test(cur) || cur === '(';
      if (prevEnds && curStarts) out.push('*');
    }
    out.push(cur);
  }

  let pos = 0;
  const peek = () => out[pos];
  const next = () => out[pos++];

  function parsePrimary(): number {
    const t = next();
    if (t === '(') {
      const v = parseExpr();
      if (next() !== ')') throw new Error('unbalanced parens');
      return v;
    }
    if (t === '-') return -parsePrimary();
    if (t === '+') return parsePrimary();
    if (/^\d/.test(t)) return parseFloat(t);
    if (/^[a-zA-Z_]/.test(t)) {
      if (t in symbols) return symbols[t];
      throw new Error(`unknown symbol: ${t}`);
    }
    throw new Error(`unexpected token: ${t}`);
  }

  function parsePower(): number {
    let base = parsePrimary();
    while (peek() === '^') {
      next();
      base = Math.pow(base, parsePower());
    }
    return base;
  }

  function parseTerm(): number {
    let v = parsePower();
    while (peek() === '*' || peek() === '/') {
      const op = next();
      const rhs = parsePower();
      v = op === '*' ? v * rhs : v / rhs;
    }
    return v;
  }

  function parseExpr(): number {
    let v = parseTerm();
    while (peek() === '+' || peek() === '-') {
      const op = next();
      const rhs = parseTerm();
      v = op === '+' ? v + rhs : v - rhs;
    }
    return v;
  }

  const result = parseExpr();
  if (pos < out.length) throw new Error('trailing tokens');
  return result;
}

// Re-evaluate all assignments top-down. Returns updated table + error line idx (-1 = ok).
export function evalAssignments(
  lines: string[],
  prev: SymbolTable = {},
): { symbols: SymbolTable; errorIdx: number } {
  const symbols: SymbolTable = { ...prev };
  for (let i = 0; i < lines.length; i++) {
    const a = parseAssignment(lines[i]);
    if (!a) continue;
    try {
      symbols[a.name] = evalExpr(a.rhs, symbols);
    } catch {
      return { symbols, errorIdx: i };
    }
  }
  return { symbols, errorIdx: -1 };
}

// Substitute symbol identifiers in an expression with their values.
// Preserves x and y (graph axes).
export function substituteSymbols(expr: string, symbols: SymbolTable): string {
  return expr.replace(/\b([a-zA-Z_]\w*)\b/g, (m) => {
    if (m === 'x' || m === 'y') return m;
    return m in symbols ? String(symbols[m]) : m;
  });
}

// Try to evaluate a non-assignment line as pure number (for display badge).
export function evalConstExpr(line: string, symbols: SymbolTable): number | null {
  if (parseAssignment(line)) return null;
  const clean = line.trim();
  if (!clean) return null;
  try {
    const v = evalExpr(clean, symbols);
    return Number.isFinite(v) ? v : null;
  } catch {
    return null;
  }
}
