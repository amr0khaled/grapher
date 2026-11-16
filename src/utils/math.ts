// Sampling via Rust WASM engine + grid helpers
import { sample } from '../wasm/grapher_engine.js';

// Build SVG path from flat [px, py, ...] buffer; NaN pairs break segments.
export function sampleToPath(
  expr: string,
  scale: number,
  offsetX: number,
  offsetY: number,
  width = 800,
  height = 600,
): string {
  const clean = expr.trim();
  if (!clean) return '';
  const flat = sample(clean, scale, offsetX, offsetY, width, height) as Float64Array;
  let d = '';
  let pen = false;
  for (let i = 0; i < flat.length; i += 2) {
    const px = flat[i];
    const py = flat[i + 1];
    if (Number.isNaN(px) || Number.isNaN(py)) {
      pen = false;
      continue;
    }
    d += pen
      ? ` L ${px.toFixed(1)} ${py.toFixed(1)}`
      : ` M ${px.toFixed(1)} ${py.toFixed(1)}`;
    pen = true;
  }
  return d;
}

export interface GridLine {
  pos: number;
  label: number;
  isMajor: boolean;
}

// Nice 1/2/5 grid steps across [0, max] screen px
export function generateGridLines(
  scale: number,
  offset: number,
  max: number,
): GridLine[] {
  const targetPx = 48;
  const raw = targetPx / scale;
  const pow = Math.pow(10, Math.floor(Math.log10(raw)));
  const mult = raw / pow;
  const worldStep = (mult >= 5 ? 5 : mult >= 2 ? 2 : 1) * pow;

  const lines: GridLine[] = [];
  const wStart = Math.ceil(-offset / scale / worldStep) * worldStep;
  const wEnd = (max - offset) / scale;
  for (let w = wStart; w <= wEnd + 1e-9; w += worldStep) {
    if (Math.abs(w) < 1e-9) continue; // skip origin line
    const wrapped = Math.abs(w % (worldStep * 5)) < 1e-9;
    lines.push({
      pos: offset + w * scale,
      label: +w.toFixed(10),
      isMajor: wrapped,
    });
  }
  return lines;
}
