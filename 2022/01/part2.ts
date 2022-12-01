import { readFileSync } from 'fs';
import { basename } from 'path';
import { part2 } from './solution';

const input = readFileSync(`./${basename(__dirname)}/input.txt`).toString();

console.log(part2(input));