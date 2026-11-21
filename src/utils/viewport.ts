export function preserveCenterOnResize(
  prevW: number,
  prevH: number,
  newW: number,
  newH: number,
  scale: number,
  offsetX: number,
  offsetY: number
) {
  const dx = (newW - prevW) / 2;
  const dy = (newH - prevH) / 2;
  return {
    offsetX: offsetX + dx,
    offsetY: offsetY + dy,
  };
}
