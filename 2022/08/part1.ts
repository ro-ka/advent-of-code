import { readFileSync } from 'fs';
import { basename } from 'path';
import { part1 } from './solution';

const input = readFileSync(`./${basename(__dirname)}/input.txt`).toString();

console.log(part1(input));