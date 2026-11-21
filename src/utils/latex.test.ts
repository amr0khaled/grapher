import { describe, expect, test } from "vitest";
import { toPlainInfix } from "./latex";

describe("PlanInfix Fractions", () => {
  test('evaluates single digits', () => {
    expect(toPlainInfix("\\frac13")).toBe("(1)/(3)")
    expect(toPlainInfix("\\frac02")).toBe("(0)/(2)")
    expect(toPlainInfix("\\frac91")).toBe("(9)/(1)")
    expect(toPlainInfix("\\frac9c")).toBe("(9)/(c)")
  });
  test('evaluates single brackets', () => {
    expect(toPlainInfix("\\frac{1}3")).toBe("(1)/(3)")
    expect(toPlainInfix("\\frac0{2}")).toBe("(0)/(2)")
    expect(toPlainInfix("\\frac{9}1")).toBe("(9)/(1)")
    expect(toPlainInfix("\\frac9{c}")).toBe("(9)/(c)")
  });
  test('evaluates brackets', () => {
    expect(toPlainInfix("\\frac{1}{3}")).toBe("(1)/(3)")
    expect(toPlainInfix("\\frac{0}{2}")).toBe("(0)/(2)")
    expect(toPlainInfix("\\frac{9}{1}")).toBe("(9)/(1)")
    expect(toPlainInfix("\\frac{9}{c}")).toBe("(9)/(c)")
  });
})
