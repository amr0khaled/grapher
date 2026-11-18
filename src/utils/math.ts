// Sampling via Rust WASM engine + grid helpers
import initEngine, { sample } from '../wasm/grapher_engine.js';
import { toPlainInfix } from './latex';



// Module evaluation waits for engine init, so `sample` is always safe to call,
// even across HMR re-instantiation of the WASM module.
try {
  await initEngine();
} catch (e) {
  console.error("WASM engine init failed", e);
}

// Build SVG path from one line's flat [px, py, ...] buffer; NaN pairs break segments.
function pathFromFlat(flat: Float64Array): string {
  let d = '';
  let pen = false;
  for (let i = 0; i + 1 < flat.length; i += 2) {
    const x = flat[i];
    const y = flat[i + 1];
    if (!Number.isFinite(x) || !Number.isFinite(y)) {
      pen = false;
      continue;
    }
    d += (pen ? 'L' : 'M') + x.toFixed(2) + ' ' + y.toFixed(2);
    pen = true;
  }
  return d;
}

// Sample all lines in one batch; returns index-aligned SVG path strings.
export function sampleAllToPaths(
  expressions: string[],
  scale: number,
  offsetX: number,
  offsetY: number,
  width = 800,
  height = 600,
): string[] {
  const plain = expressions.map((e) => toPlainInfix(e));
  const arrays = sample(plain, scale, offsetX, offsetY, width, height) as unknown[];
  const out: string[] = [];
  for (let i = 0; i < arrays.length; i++) {
    const flat = arrays[i] as Float64Array;
    out.push(pathFromFlat(flat));
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
