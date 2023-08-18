import { main } from './gol.ts';

describe('Game of Life', () => {
  it('says hello, world', () => {
    const spy = jest.spyOn(console, 'log').mockImplementationOnce(() => {});
    main();
    expect(spy).toHaveBeenCalledWith('hello, world');
  });
});
