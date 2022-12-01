import { expect, test } from 'bun:test';
import { readFileSync } from 'fs';

import { part1, part2 } from './solution';

const exampleInput = readFileSync('./01/example.txt').toString();

test('part1 is correct', () => {
  expect(part1(exampleInput)).toBe(24000);
})

test('part2 is correct', () => {
  expect(part2(exampleInput)).toBe(45000);
})