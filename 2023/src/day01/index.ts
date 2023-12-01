import run from "aocrunner";

const parseInput = (rawInput: string) => rawInput.split("\n");

const part1 = (rawInput: string) => {
  const input = parseInput(rawInput);

  return String(
    input
      .map((row) => row.match(/\d{1}/g))
      .map((numbers) => Number(`${numbers[0]}${numbers[numbers?.length - 1]}`))
      .reduce((sum, number) => (sum += number), 0),
  );
};

const part2 = (rawInput: string) => {
  const input = parseInput(rawInput);

  return String(
    input
      .map((row) =>
        row.match(/(one|two|three|four|five|six|seven|eight|nine|\d{1})/g),
      )
      .map((row) =>
        row.map((number) =>
          Number(
            number
              .replace("one", "1")
              .replace("two", "2")
              .replace("three", "3")
              .replace("four", "4")
              .replace("five", "5")
              .replace("six", "6")
              .replace("seven", "7")
              .replace("eight", "8")
              .replace("nine", "9"),
          ),
        ),
      )
      .map((numbers) => Number(`${numbers[0]}${numbers[numbers?.length - 1]}`))
      .reduce((sum, number) => (sum += number), 0),
  );
};

run({
  part1: {
    tests: [
      {
        input: `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`,
        expected: "142",
      },
    ],
    solution: part1,
  },
  part2: {
    tests: [
      {
        input: `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`,
        expected: "281",
      },
    ],
    solution: part2,
  },
  trimTestInputs: true,
  onlyTests: false,
});
