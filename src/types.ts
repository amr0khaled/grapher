export interface ExpressionLine {
  expression: string;
  color: string;
  visible: boolean;
}

export interface GridLine {
  pos: number;
  label: number;
  isMajor: boolean;
}

export interface LatexImage {
  img: HTMLImageElement;
  width: number; // CSS px
  height: number;
}
