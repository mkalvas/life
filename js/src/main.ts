import fs from 'fs';
import { Game } from './game.ts';

export const main = async () => {
  let init = fs.readFileSync('../patterns/gosper-glider.txt', 'utf8');
  const game = new Game(init);
  while (true) {
    console.clear();
    console.log(`${game}`);
    await wait(50);
    game.step();
  }
};

// @ts-ignore
const waitForKey = () => {
  return new Promise((resolve) => {
    process.stdin.resume();
    process.stdin.once('data', (data) => {
      process.stdin.pause();
      resolve(data.toString());
    });
  });
};

const wait = (ms: number) => {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
};
