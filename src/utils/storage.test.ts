import { describe, it, expect } from "vitest";
import { encodeShareUrl, decodeShareUrl } from "./storage";

describe("URL state encoding", () => {
  it("encodes and decodes expression state without loss", () => {
    const state = {
      expressions: [{ expression: "y=\\sin(x)", color: "#10b981", visible: true }],
      scale: 50,
      offsetX: 400,
      offsetY: 300,
      showGrid: true,
      darkMode: true,
    };
    const hash = encodeShareUrl(state);
    const decoded = decodeShareUrl(hash);
    expect(decoded?.expressions[0].expression).toBe("y=\\sin(x)");
    expect(decoded?.scale).toBe(50);
  });
});
