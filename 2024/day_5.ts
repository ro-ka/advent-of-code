import { assertEquals } from "@std/assert";
import { runPart } from "@macil/aocd";

function parse(input: string) {
  const raw = input.trimEnd().split("\n\n");

  return {
    orderRules: raw[0].split('\n').map(row => row.split('|').map(Number)),
    updates: raw[1].split('\n').map(row => row.split(',').map(Number))
  }
}

function getOrderRuleIndices(rule: number[], update: number[]) {
  const firstPageIndex = update.findIndex(page => page === rule[0]);
  const secondPageIndex = update.findIndex(page => page === rule[1]);

  return {firstPageIndex, secondPageIndex};
}

function orderRuleApplies(firstPageIndex: number, secondPageIndex: number) {
  return firstPageIndex >= 0 && secondPageIndex >= 0;
}

function orderRuleIsFulfilled(rule: number[], update: number[]): boolean {
  const {firstPageIndex, secondPageIndex} = getOrderRuleIndices(rule, update);

  if (!orderRuleApplies(firstPageIndex, secondPageIndex)) return true;

  return firstPageIndex < secondPageIndex;
}

function isCorrectlyOrdered(orderRules: number[][], update: number[]): boolean {
  return orderRules.every(rule => orderRuleIsFulfilled(rule, update));
}

function fixUpdate(orderRules: number[][], update: number[]): number[] {
  let newUpdate = update;
  let indexToMove = 0;
  let newIndex = 1;

  while (!isCorrectlyOrdered(orderRules, newUpdate)) {

  }

  return update;
}

function getMiddlePagesSum(updates: number[][]): number {
  return updates
    .map(update => update[Math.floor(update.length / 2)])
    .reduce((total: number, page) => total += page, 0);
}

function part1(input: string): number {
  const {orderRules, updates} = parse(input);
  const correctUpdates = updates.filter(update => isCorrectlyOrdered(orderRules, update));
  return getMiddlePagesSum(correctUpdates);
}

function part2(input: string): number {
  const {orderRules, updates} = parse(input);

  const incorrectUpdates = updates
    .filter(update => !isCorrectlyOrdered(orderRules, update))
    .map(update => fixUpdate(orderRules, update));

  return getMiddlePagesSum(incorrectUpdates);
}

if (import.meta.main) {
  runPart(2024, 5, 1, part1);
  runPart(2024, 5, 2, part2);
}

const TEST_INPUT = `\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
`;

Deno.test("part1", () => {
  assertEquals(part1(TEST_INPUT), 143);
});

Deno.test("part2", () => {
  assertEquals(part2(TEST_INPUT), 123);
});
