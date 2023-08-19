export const FILLED = 'o';
export const EMPTY = ' ';

export class Cell {
  public on: boolean = false;
  public next: boolean = false;

  constructor(alive = false) {
    this.on = alive;
    this.next = alive;
  }

  toString(): string {
    return this.on ? FILLED : EMPTY;
  }

  to(to: boolean) {
    this.next = to;
  }

  flush() {
    this.on = this.next;
  }
}
