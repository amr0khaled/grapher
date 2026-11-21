import { describe, it, expect } from "vitest";
import { GestureTracker } from "./gestureController";

describe("GestureTracker", () => {
  it("tracks two pointers and computes midpoint and distance", () => {
    const tracker = new GestureTracker();
    tracker.addPointer(1, 100, 200);
    tracker.addPointer(2, 300, 200);

    const pinch = tracker.getPinchInfo();
    expect(pinch.center).toEqual({ x: 200, y: 200 });
    expect(pinch.distance).toBe(200);
  });
});
