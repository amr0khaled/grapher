import { describe, it, expect } from "vitest";
import { preserveCenterOnResize } from "./viewport";

describe("preserveCenterOnResize", () => {
  it("keeps world center fixed when canvas dimensions change", () => {
    // Initial 800x600 with center at (400, 300) in screen coords
    const initial = {
      prevW: 800,
      prevH: 600,
      newW: 1000,
      newH: 800,
      scale: 50,
      offsetX: 400,
      offsetY: 300,
    };
    const result = preserveCenterOnResize(
      initial.prevW,
      initial.prevH,
      initial.newW,
      initial.newH,
      initial.scale,
      initial.offsetX,
      initial.offsetY
    );
    // New center should be at (500, 400) screen coords
    expect(result.offsetX).toBe(500);
    expect(result.offsetY).toBe(400);
  });
});
