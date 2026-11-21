// Sampling via Rust WASM engine + grid helpers
import initEngine, { get_symbol_values, sample, is_drawable } from '../wasm/grapher_engine.js';
import { toPlainInfix } from './latex';



// Module evaluation waits for engine init, so `sample` is always safe to call,
// even across HMR re-instantiation of the WASM module.
try {
  await initEngine();
} catch (e) {
  console.error("WASM engine init failed", e);
}

// Evaluate symbol values from expressions
export function evaluateSymbols(expressions: string[]): Record<string, number> {
  const plain = expressions.map((e) => toPlainInfix(e));
  return get_symbol_values(plain) as Record<string, number>;
}

// Build Path2D from one line's flat [px, py, ...] buffer; NaN pairs break segments.
function buildPath2D(flat: Float64Array): Path2D | null {
  if (!flat || flat.length < 2) return null;
  const p = new Path2D();
  let hasPoints = false;
  let startedSubpath = false;
  for (let i = 0; i + 1 < flat.length; i += 2) {
    const x = flat[i];
    const y = flat[i + 1];
    if (!Number.isFinite(x) || !Number.isFinite(y)) {
      startedSubpath = false;
      continue;
    }
    hasPoints = true;
    if (!startedSubpath) {
      p.moveTo(x, y);
      startedSubpath = true;
    } else {
      p.lineTo(x, y);
    }
  }
  return hasPoints ? p : null;
}

// Classify whether an expression should be drawn on the canvas.
// Definitions (a = 5) return false; y=..., x=..., f(x)=..., implicit lhs=rhs return true.
export function isDrawable(expression: string): boolean {
  const plain = toPlainInfix(expression);
  return is_drawable(plain);
}

// Evaluate variable assignments with determinable values.
// Returns numerical result or null if RHS cannot be completely resolved.
export function getEvaluatedValue(
  expression: string,
  symbols: Record<string, number> = {},
): number | null {
  const plain = toPlainInfix(expression).trim();
  const assignment = plain.match(/^\s*([a-zA-Z_]\w*)\s*=\s*(.+)$/);
  if (!assignment) return null;

  const varName = assignment[1];
  const rhsExpr = assignment[2].trim();

  // Pure numeric literal
  if (/^[+-]?\d+(\.\d+)?$/.test(rhsExpr)) {
    return parseFloat(rhsExpr);
  }

  // Already computed in symbol table
  if (symbols[varName] !== undefined && !Number.isNaN(symbols[varName])) {
    return symbols[varName];
  }

  // Symbol reference
  if (symbols[rhsExpr] !== undefined && !Number.isNaN(symbols[rhsExpr])) {
    return symbols[rhsExpr];
  }

  return null;
}

// Sample all lines in one batch; returns index-aligned Path2D objects.
export function sampleToPath2D(
  expressions: string[],
  scale: number,
  offsetX: number,
  offsetY: number,
  width: number,
  height: number,
): (Path2D | null)[] {
  const plain = expressions.map((e) => toPlainInfix(e));
  const arrays = sample(plain, scale, offsetX, offsetY, width, height) as unknown[];
  const out: (Path2D | null)[] = [];
  for (let i = 0; i < arrays.length; i++) {
    const flat = arrays[i] as Float64Array;
    out.push(buildPath2D(flat));
  }
  return out;
}

export interface GridLine {
  pos: number;
  label: number;
  isMajor: boolean;
}

// Smallest integer nice step >= raw (1, 2, 5, 10, 20, 50, 100 ...)
function niceStep(raw: number): number {
  if (raw <= 1) return 1;
  const pow = Math.pow(10, Math.floor(Math.log10(raw) + 1e-12));
  const mult = raw / pow;
  const step = (mult >= 5 ? 5 : mult >= 2 ? 2 : 1) * pow;
  return Math.max(1, Math.round(step));
}

export interface PinnedAxis {
  axisX: number;
  axisY: number;
  labelAlignX: "left" | "right" | "center";
  labelAlignY: "top" | "bottom" | "middle";
}

export function getPinnedAxisPositions(
  offsetX: number,
  offsetY: number,
  width: number,
  height: number,
  padding: number
): PinnedAxis {
  const axisX = Math.max(padding, Math.min(width - padding, offsetX));
  const axisY = Math.max(padding, Math.min(height - padding, offsetY));

  const labelAlignX: "left" | "right" | "center" =
    offsetX < padding ? "left" : offsetX > width - padding ? "right" : "left";

  const labelAlignY: "top" | "bottom" | "middle" =
    offsetY < padding ? "top" : offsetY > height - padding ? "bottom" : "middle";

  return { axisX, axisY, labelAlignX, labelAlignY };
}

export function generateGridLines(
  scale: number,
  offset: number,
  max: number,
): GridLine[] {
  const targetPx = 80;
  const raw = targetPx / scale;
  let exp = Math.floor(Math.log10(raw));
  let pow = Math.pow(10, exp);
  const mult = raw / pow;

  // Discrete nice step selection (1, 2, 5, 10...)
  let baseMult = 1;
  if (mult >= 5) baseMult = 5;
  else if (mult >= 2) baseMult = 2;
  else baseMult = 1;

  let worldStep = baseMult * pow;

  // Ensure major step never goes below targetPx (80px) on screen, preventing continuous step mutation and label jumping (e.g. 1 -> 0.8)
  while (worldStep * scale < 80) {
    if (baseMult === 1) { baseMult = 2; }
    else if (baseMult === 2) { baseMult = 5; }
    else { baseMult = 1; pow *= 10; exp++; }
    worldStep = baseMult * pow;
  }

  const subdiv = baseMult === 5 ? 5 : (baseMult === 2 ? 4 : 5);
  const minorStep = worldStep / subdiv;
  // If minor step is getting too tight, hide minor lines earlier to prevent flashing
  const showMinor = minorStep * scale >= 20;

  const step = showMinor ? minorStep : worldStep;
  const lines: GridLine[] = [];
  const ratio = Math.round(worldStep / step);

  const kStart = Math.ceil(-offset / scale / step);
  const kEnd = Math.floor((max - offset) / scale / step);
  for (let k = kStart; k <= kEnd; k++) {
    const w = k * step;
    if (Math.abs(w) < step * 1e-9) continue;
    const isMajor = Math.abs(k % ratio) === 0;
    lines.push({
      pos: offset + w * scale,
      label: +w.toFixed(10),
      isMajor,
    });
  }
  return lines;
}
