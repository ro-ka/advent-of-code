import { expect, test } from 'bun:test';
import { readFileSync } from 'fs';
import { part1, part2 } from './solution';
import { basename } from 'path';

const exampleInput = readFileSync(`./${basename(__dirname)}/example.txt`).toString();

test('part1 is correct', () => {
  expect(part1(exampleInput)).toBe(15);
})

test('part2 is correct', () => {
  expect(part2(exampleInput)).toBe(12);
})