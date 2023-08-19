export const FILLED = '█';
export const EMPTY = ' ';

export class Cell {
  public on: boolean = false;

  constructor(alive = false) {
    this.on = alive;
  }

  toString(): string {
    return this.on ? FILLED : EMPTY;
  }

  toggle() {
    this.on = !this.on;
  }
}
