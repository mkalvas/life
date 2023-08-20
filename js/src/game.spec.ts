import { EMPTY, FILLED } from './cell.ts';
import { Game } from './game.ts';

const scenarios = [
  { from: ' ', to: ' ' },
  { from: ' O ', to: '   ' },
  { from: 'O O', to: '   ' },
  { from: 'OOO', to: ' O ' },
  { from: ' O \nO O\n   ', to: ' O \n O \n   ' },
  { from: 'OOO\nOOO\nOOO', to: 'O O\n   \nO O' },
  { from: '    \n OO \n OO \n    ', to: '    \n OO \n OO \n    ' },
];

describe('Game', () => {
  it('prints the given configuration', () => {
    const game = new Game('O O');
    expect(`${game}`).toBe(`${FILLED}${EMPTY}${FILLED}`);
  });

  it.each(scenarios)('evolves from \n$from\n to \n$to\n', ({ from, to }) => {
    const game = new Game(from);
    game.step();
    expect(`${game}`).toBe(to);
  });
});
