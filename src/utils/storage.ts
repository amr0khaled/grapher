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

export const encodeShareUrl = (state: AppState): string => {
  try {
    const json = JSON.stringify(state);
    return btoa(encodeURIComponent(json));
  } catch (e) {
    return "";
  }
};

export const decodeShareUrl = (hash: string): AppState | null => {
  try {
    const clean = hash.startsWith("#") ? hash.slice(1) : hash;
    if (!clean) return null;
    const json = decodeURIComponent(atob(clean));
    return JSON.parse(json);
  } catch (e) {
    return null;
  }
};

