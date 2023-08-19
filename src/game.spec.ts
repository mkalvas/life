import { Cell, EMPTY, FILLED } from './cell.ts';
import { Game } from './game.ts';

describe('Game', () => {
  it('prints the given configuration', () => {
    const game = new Game([[new Cell(true), new Cell(), new Cell(true)]]);
    expect(`${game}`).toBe(`${FILLED}${EMPTY}${FILLED}`);
  });
});
