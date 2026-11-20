import { describe, it, expect } from "vitest";
import { findNearestCurvePoint } from "./curveTrace";

describe("findNearestCurvePoint", () => {
  it("finds closest point on sampled curve within threshold", () => {
    // Screen buffer: (100, 200), (105, 210), (110, 220)
    const buf = new Float64Array([100, 200, 105, 210, 110, 220]);
    // Tap near (105, 210) at (107, 212) -> distance is ~2.8px
    const hit = findNearestCurvePoint([buf], 107, 212, 50, 400, 300, 15);
    expect(hit).not.toBeNull();
    expect(hit?.screenX).toBe(105);
    expect(hit?.screenY).toBe(210);
    expect(hit?.exprIndex).toBe(0);
  });

  it("returns null if tap is further than maxDistPx", () => {
    const buf = new Float64Array([100, 200]);
    const hit = findNearestCurvePoint([buf], 200, 200, 50, 400, 300, 15);
    expect(hit).toBeNull();
  });
});
