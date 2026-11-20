export interface TracedPoint {
  worldX: number;
  worldY: number;
  screenX: number;
  screenY: number;
  exprIndex: number;
}

export function findNearestCurvePoint(
  flatBuffers: (Float64Array | null | undefined)[],
  tapScreenX: number,
  tapScreenY: number,
  scale: number,
  offsetX: number,
  offsetY: number,
  maxDistPx = 20
): TracedPoint | null {
  let closestDist = maxDistPx;
  let closestPoint: TracedPoint | null = null;

  for (let exprIdx = 0; exprIdx < flatBuffers.length; exprIdx++) {
    const buf = flatBuffers[exprIdx];
    if (!buf || buf.length < 2) continue;

    for (let i = 0; i + 1 < buf.length; i += 2) {
      const px = buf[i];
      const py = buf[i + 1];
      if (!Number.isFinite(px) || !Number.isFinite(py)) continue;

      const dist = Math.hypot(px - tapScreenX, py - tapScreenY);
      if (dist < closestDist) {
        closestDist = dist;
        closestPoint = {
          screenX: px,
          screenY: py,
          worldX: +((px - offsetX) / scale).toFixed(6),
          worldY: -+((py - offsetY) / scale).toFixed(6),
          exprIndex: exprIdx,
        };
      }
    }
  }

  return closestPoint;
}
