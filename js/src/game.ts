import { Cell, FILLED } from './cell.ts';

export class Game {
  private state: Cell[][] = [];

  constructor(state: string = '') {
    this.state = state
      .split('\n')
      .map((r) => r.split('').map((c) => new Cell(c === FILLED)));
  }

  toString() {
    return this.state.map((r) => r.map((c) => `${c}`).join('')).join('\n');
  }

  map(fn: (cell: Cell, x: number, y: number) => void) {
    for (const [y, row] of this.state.entries()) {
      for (let [x, cell] of row.entries()) {
        fn(cell, x, y);
      }
    }
  }

  step() {
    this.map((cell, x, y) => {
      let nbrs = this.countNeighbors(x, y);
      // Any dead cell with three live neighbours becomes a live cell.
      if (!cell.on && nbrs === 3) {
        cell.to(true);
      } else if (cell.on && (nbrs < 2 || nbrs > 3)) {
        // Any live cell with two or three live neighbours survives.
        // All other live cells die in the next generation.
        cell.to(false);
      }
      // All other dead cells stay dead.
    });

    this.map((cell) => cell.flush());
  }

  countNeighbors(x: number, y: number) {
    let nbrs = 0;
    for (const dr of [-1, 0, 1]) {
      for (const dc of [-1, 0, 1]) {
        const r = y + dr;
        const c = x + dc;
        if (
          r >= 0 &&
          c >= 0 &&
          r < this.state.length &&
          c < (this.state?.[y]?.length ?? -Infinity) &&
          !(dr === 0 && dc === 0) &&
          this.state[r]?.[c]?.on
        ) {
          nbrs += 1;
        }
      }
    }
    return nbrs;
  }
}
