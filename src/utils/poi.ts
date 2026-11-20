export interface POI {
  x: number;
  y: number;
  type: 'root' | 'extremum';
}

export function findPointsOfInterest(flatBuffer: Float64Array): POI[] {
  const pois: POI[] = [];
  if (!flatBuffer || flatBuffer.length < 6) return pois;

  for (let i = 2; i < flatBuffer.length - 2; i += 2) {
    const x0 = flatBuffer[i - 2], y0 = flatBuffer[i - 1];
    const x1 = flatBuffer[i], y1 = flatBuffer[i + 1];
    const x2 = flatBuffer[i + 2], y2 = flatBuffer[i + 3];

    if (!Number.isFinite(y0) || !Number.isFinite(y1) || !Number.isFinite(y2)) continue;

    if (y0 * y1 < 0) {
      const rx = x0 + (x1 - x0) * (0 - y0) / (y1 - y0);
      pois.push({ x: rx, y: 0, type: 'root' });
    }

    const dy1 = y1 - y0;
    const dy2 = y2 - y1;
    if (dy1 * dy2 < 0) {
      pois.push({ x: x1, y: y1, type: 'extremum' });
    }
  }
  return pois;
}
