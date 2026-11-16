// Default Desmos-style colors
export const DEFAULT_COLORS: string[] = [
  '#388c46', // green
  '#fa7e19', // orange
  '#6042a6', // purple
  '#c1232b', // red
  '#0070c0', // blue
  '#e67e22', // amber
  '#2c3e50', // slate
  '#9c27b0', // magenta
];

export function generateColor(index: number): string {
  return DEFAULT_COLORS[index % DEFAULT_COLORS.length];
}

function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
  const m = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return m ? { r: parseInt(m[1], 16), g: parseInt(m[2], 16), b: parseInt(m[3], 16) } : null;
}

function rgbToHex(r: number, g: number, b: number): string {
  return '#' + [r, g, b].map((x) => x.toString(16).padStart(2, '0')).join('');
}

export function getContrastColor(hex: string): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return '#000';
  const luminance = (0.299 * rgb.r + 0.587 * rgb.g + 0.114 * rgb.b) / 255;
  return luminance > 0.5 ? '#000' : '#fff';
}
