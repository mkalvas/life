import { main } from './main.ts';

describe('Main', () => {
  xit('says hello, world', () => {
    const spy = jest.spyOn(console, 'log').mockImplementationOnce(() => {});
    main();
    expect(spy).toHaveBeenCalledWith('');
  });
});
