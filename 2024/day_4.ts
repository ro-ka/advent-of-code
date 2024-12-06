import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

type Step = -1 | 0 | 1;

function parse(input: string) {
  return input.trimEnd().split("\n");
}

function getCharIndices(row: string, char: string) {
  const regex = new RegExp(char, 'g');
  const indices: number[] = [];
  let result: RegExpExecArray | null = null;

  while ((result = regex.exec(row))) {
    indices.push(result.index);
  }

  return indices;
}

function isXmas(
  rows: string[],
  rowIndex: number,
  xIndex: number,
  steps: {x: Step, y: Step}
) {
  return [0, 1, 2, 3]
    .map(position => {
      const newRowIndex = rowIndex + position * steps.y;
      if (newRowIndex < 0 || newRowIndex >= rows.length) return '?';
      return rows[newRowIndex].charAt(xIndex + position * steps.x);
    })
    .join('') === 'XMAS';
}

function countXmasForPosition(rows: string[], rowIndex: number, xIndex: number) {
  let xmasCounter = 0;

  for (const x of [-1, 0, 1] as Step[]) {
    for (const y of [-1, 0, 1] as Step[]) {
      if (isXmas(rows, rowIndex, xIndex, {x, y})) {
        xmasCounter += 1;
      }
    }
  }

  return xmasCounter;
}

function isXmasShape(rows: string[], rowIndex: number, aIndex: number) {
  if (rowIndex - 1 < 0 || rowIndex + 1 >= rows.length) return false;

  const direction1 = `${rows[rowIndex - 1].charAt(aIndex - 1)}${rows[rowIndex + 1].charAt(aIndex + 1)}`;
  const direction2 = `${rows[rowIndex - 1].charAt(aIndex + 1)}${rows[rowIndex + 1].charAt(aIndex - 1)}`;

  if (
    (direction1 === 'MS' || direction1 === 'SM') &&
    (direction2 === 'MS' || direction2 === 'SM')
  ) {
    return true;
  }

  return false;
}

function part1(input: string): number {
  const rows = parse(input);

  return rows.reduce((total, row, rowIndex) => {
    const xIndices = getCharIndices(row, 'X');
    return total += xIndices
      .map(xIndex => countXmasForPosition(rows, rowIndex, xIndex))
      .reduce((sum: number, countForPosition) => sum += countForPosition, 0);
  }, 0);
}

function part2(input: string): number {
  const rows = parse(input);

  return rows.reduce((total, row, rowIndex) => {
    const aIndices = getCharIndices(row, 'A');

    return total += aIndices
      .map(aIndex => isXmasShape(rows, rowIndex, aIndex))
      .reduce((sum: number, isXmas) => {
        if (isXmas) return sum += 1;
        return sum;
      }, 0);
  }, 0);


  throw new Error("TODO");
}

if (import.meta.main) {
  runPart(2024, 4, 1, part1);
  runPart(2024, 4, 2, part2);
}

const TEST_INPUT = `\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 18);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 9);
});
