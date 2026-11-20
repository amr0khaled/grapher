import type { ExpressionLine } from "../types";

export interface AppState {
  expressions: ExpressionLine[];
  scale: number;
  offsetX: number;
  offsetY: number;
  showGrid: boolean;
  darkMode: boolean;
}

export const saveState = (state: AppState) => {
  try {
    localStorage.setItem("grapher_state_v1", JSON.stringify(state));
  } catch (e) {
    console.error("Failed to save state", e);
  }
};

export const loadState = (): AppState | null => {
  try {
    const saved = localStorage.getItem("grapher_state_v1");
    return saved ? JSON.parse(saved) : null;
  } catch (e) {
    console.error("Failed to load state", e);
    return null;
  }
};
