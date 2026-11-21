export function resolveLayoutMode(
  width: number,
  height: number,
  hasHover: boolean,
  hasFinePointer: boolean
): "desktop" | "tablet" | "mobile" | "landscape-mobile" {
  if (width >= 1024 && hasHover) return "desktop";
  if (width < 640) return "mobile";
  if (width < 900 && height < 500 && height < width) return "landscape-mobile";
  return "tablet";
}
