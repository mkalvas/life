import { Cell } from './cell.ts';

/**
 * Any live cell with two or three live neighbours survives.
 * Any dead cell with three live neighbours becomes a live cell.
 * All other live cells die in the next generation. Similarly, all other dead cells stay dead.
 */

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

export class Game {
  private state: Cell[][] = [];

  constructor(state: Cell[][] = []) {
    this.state = state;
  }

  toString() {
    return this.state.map((r) => r.map((c) => `${c}`).join('')).join('\n');
  }
}
