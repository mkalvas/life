import { Cell, EMPTY, FILLED } from './cell.ts';

describe('Cell', () => {
  it('can be initialized alive or dead', () => {
    const dead1 = new Cell();
    const dead2 = new Cell(false);
    const alive = new Cell(true);

    expect(alive.on).toBe(true);
    expect(dead1.on).toBe(false);
    expect(dead2.on).toBe(false);
  });

  it('can make the cell dead', () => {
    const cell = new Cell(true);
    cell.toggle();
    expect(cell.on).toBe(false);
  });

  it('can make the cell live', () => {
    const cell = new Cell();
    cell.toggle();
    expect(cell.on).toBe(true);
  });

  it('has a string representation', () => {
    const dead = new Cell();
    const alive = new Cell(true);
    expect(`${alive}`).toBe(FILLED);
    expect(dead.toString()).toBe(EMPTY);
  });
});
