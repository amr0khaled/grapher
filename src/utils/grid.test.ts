import { describe, it, expect } from "vitest";
import { generateGridLines, getPinnedAxisPositions } from "./math";

describe("generateGridLines", () => {
  it("generates major and minor lines", () => {
    const lines = generateGridLines(100, 400, 800);
    expect(lines.length).toBeGreaterThan(0);
    expect(lines.some(l => l.isMajor)).toBe(true);
  });

  it("calculates correct labels", () => {
    const lines = generateGridLines(100, 400, 800);
    const one = lines.find(l => Math.abs(l.label - 1) < 1e-9);
    expect(one).toBeDefined();
    expect(one?.pos).toBe(500);
  });
});

describe("getPinnedAxisPositions", () => {
  it("pins axes to screen borders when origin is offscreen", () => {
    const result = getPinnedAxisPositions(-100, 1000, 800, 600, 20);
    expect(result.axisX).toBe(20);
    expect(result.axisY).toBe(580);
  });
});

