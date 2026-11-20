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
