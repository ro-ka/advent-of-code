import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  return input.trimEnd();
}

function calculate(input: string): number {
  return [...input.matchAll(/mul\((\d+),(\d+)\)/g)]
    .map(match => Number(match[1]) * Number(match[2]))
    .reduce((total: number, mul) => total += mul, 0);
}

function part1(input: string): number {
  const rawInput = parse(input);
  return calculate(rawInput);
}

function part2(input: string): number {
  const rawInput = parse(input);

  const enabledInput = rawInput.split('do()')
    .map(part => part.split("don't()"))
    .map(part => part[0])
    .join('');

  return calculate(enabledInput);
}

if (import.meta.main) {
  runPart(2024, 3, 1, part1);
  runPart(2024, 3, 2, part2);
}

const TEST_INPUT_1 = `\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
`;

const TEST_INPUT_2 = `\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT_1), 161);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT_2), 48);
});
