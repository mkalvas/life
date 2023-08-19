import { EMPTY, FILLED } from './cell.ts';
import { Game } from './game.ts';

const scenarios = [
  { from: ' ', to: ' ' },
  { from: ' o ', to: '   ' },
  { from: 'o o', to: '   ' },
  { from: 'ooo', to: ' o ' },
  { from: ' o \no o\n   ', to: ' o \n o \n   ' },
  { from: 'ooo\nooo\nooo', to: 'o o\n   \no o' },
  { from: '    \n oo \n oo \n    ', to: '    \n oo \n oo \n    ' },
];

describe('Game', () => {
  it('prints the given configuration', () => {
    const game = new Game('o o');
    expect(`${game}`).toBe(`${FILLED}${EMPTY}${FILLED}`);
  });

  it.each(scenarios)('evolves from \n$from\n to \n$to\n', ({ from, to }) => {
    const game = new Game(from);
    game.step();
    expect(`${game}`).toBe(to);
  });
});
