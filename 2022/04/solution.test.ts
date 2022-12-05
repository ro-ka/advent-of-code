import { expect, test } from 'bun:test';
import { readFileSync } from 'fs';
import { basename } from 'path';
import { part1, part2 } from './solution';

const exampleInput = readFileSync(`./${basename(__dirname)}/example.txt`).toString();

test('part1 is correct', () => {
  expect(part1(exampleInput)).toBe(2);
})

test('part2 is correct', () => {
  expect(part2(exampleInput)).toBe(4);
})