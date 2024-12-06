import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  return input.trimEnd()
    .split("\n")
    .map(row => row.split(' ').map(value => Number(value)));
}

function isIncreasing(number: number, index: number, array: number[]) {
  if (index === 0) return true;
  const distance = array[index - 1] - number;
  return [1, 2, 3].includes(distance);
};

function isDecreasing(number: number, index: number, array: number[]) {
  if (index === 0) return true;
  const distance = array[index - 1] - number;
  return [-1, -2, -3].includes(distance);
};

function isSafe(levels: number[]) {
  return levels.every(isDecreasing) || levels.every(isIncreasing)
}

function part1(input: string): number {
  const reports = parse(input);
  return reports.filter(isSafe).length;
}

function part2(input: string): number {
  const reports = parse(input);

  const safeReports = reports.filter(isSafe);
  const safeRecordsWithDampener = reports
    .filter(levels => !isSafe(levels))
    .filter(levels => {
      let isDampenedSafe = false;
      let index = 0;

      while (!isDampenedSafe && index < levels.length) {
        const dampenedLevelsTry = levels.toSpliced(index, 1);
        isDampenedSafe = isSafe(dampenedLevelsTry);
        index += 1;
      }

      return isDampenedSafe;
    });

  return safeReports.length + safeRecordsWithDampener.length;
}

if (import.meta.main) {
  runPart(2024, 2, 1, part1);
  runPart(2024, 2, 2, part2);
}

const TEST_INPUT = `\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 2);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 4);
});
