import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  return input.trimEnd()
    .split("\n")
    .map(row => row.split('   '))
    .reduce((columns: {one: number[], two: number[]}, row) => {
      columns.one.push(Number(row[0]));
      columns.two.push(Number(row[1]));
      columns.one.sort();
      columns.two.sort();
      return columns;
    }, {one: [], two: []});
}

function part1(input: string): number {
  const columns = parse(input);

  return columns.one
    .map((value1, index) => Math.abs(columns.two[index] - value1))
    .reduce((total: number, distance) => total += distance, 0);
}

function part2(input: string): number {
  const columns = parse(input);

  return columns.one
    .map((value1) => value1 * columns.two.filter(value => value === value1).length)
    .reduce((total: number, score) => total += score, 0);
}

if (import.meta.main) {
  runPart(2024, 1, 1, part1);
  runPart(2024, 1, 2, part2);
}

const TEST_INPUT = `\
3   4
4   3
2   5
1   3
3   9
3   3
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 11);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 31);
});
