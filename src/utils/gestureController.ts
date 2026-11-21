export interface Point {
  x: number;
  y: number;
}

export interface PinchInfo {
  center: Point;
  distance: number;
}

export class GestureTracker {
  private pointers = new Map<number, Point>();

  public addPointer(id: number, x: number, y: number): void {
    this.pointers.set(id, { x, y });
  }

  public updatePointer(id: number, x: number, y: number): void {
    if (this.pointers.has(id)) {
      this.pointers.set(id, { x, y });
    }
  }

  public removePointer(id: number): void {
    this.pointers.delete(id);
  }

  public getPointerCount(): number {
    return this.pointers.size;
  }

  public getPinchInfo(): PinchInfo {
    const pts = Array.from(this.pointers.values());
    if (pts.length < 2) {
      return { center: pts[0] || { x: 0, y: 0 }, distance: 0 };
    }
    const p1 = pts[0];
    const p2 = pts[1];
    const center: Point = {
      x: (p1.x + p2.x) / 2,
      y: (p1.y + p2.y) / 2,
    };
    const dx = p2.x - p1.x;
    const dy = p2.y - p1.y;
    const distance = Math.sqrt(dx * dx + dy * dy);
    return { center, distance };
  }
}
