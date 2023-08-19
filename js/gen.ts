import fs from 'fs';

let output: ('O' | '.')[][] = [];
for (let i = 0; i < 20; i++) {
  output[i] = [];
  for (let j = 0; j < 40; j++) {
    output?.[i]?.push(Math.round(Math.random()) ? 'O' : '.');
  }
}

let data = output.map((r) => r.join('')).join('\n');

fs.writeFileSync('../patterns/random.txt', data);
