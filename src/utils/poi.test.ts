import { describe, it, expect } from "vitest";
import { findPointsOfInterest } from "./poi";

describe("findPointsOfInterest", () => {
  it("detects roots and extrema", () => {
    // y values: 1, -1, 1 -> sign change between 0 and 1 (1 to -1) and 1 and 2 (-1 to 1)
    const buffer = new Float64Array([0, 1, 1, -1, 2, 1]);
    const pois = findPointsOfInterest(buffer);
    expect(pois.length).toBeGreaterThan(0);
    expect(pois.some(p => p.type === 'root')).toBe(true);
    expect(pois.some(p => p.type === 'extremum')).toBe(true);
  });
});
