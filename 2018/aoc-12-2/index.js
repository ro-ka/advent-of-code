const INITIAL = '#...##.#...#..#.#####.##.#..###.#.#.###....#...#...####.#....##..##..#..#..#..#.#..##.####.#.#.###';
const ITERATIONS = 50000000000;
const ITERATIONS_RUN = 200;
const ITERATIONS_DIFF = ITERATIONS - ITERATIONS_RUN;

const rules = [
  ['.....', '.'],
  ['..#..', '#'],
  ['..##.', '#'],
  ['#..##', '.'],
  ['..#.#', '#'],
  ['####.', '.'],
  ['##.##', '.'],
  ['#....', '.'],
  ['###..', '#'],
  ['#####', '#'],
  ['##..#', '#'],
  ['#.###', '#'],
  ['#..#.', '#'],
  ['.####', '#'],
  ['#.#..', '#'],
  ['.###.', '#'],
  ['.##..', '#'],
  ['.#...', '#'],
  ['.#.##', '#'],
  ['##...', '#'],
  ['..###', '.'],
  ['##.#.', '.'],
  ['...##', '.'],
  ['....#', '.'],
  ['###.#', '.'],
  ['#.##.', '#'],
  ['.##.#', '.'],
  ['.#..#', '#'],
  ['#.#.#', '#'],
  ['.#.#.', '#'],
  ['...#.', '#'],
  ['#...#', '#']
];

const relevantRules = rules.filter(rule => rule[0][2] !== rule[1]);

const OFFSET = 4;
const padding = ''.padStart(800, '.');
let pots = `....${INITIAL}${padding}`;

for (let iteration = 0; iteration < ITERATIONS_RUN; iteration++) {
  let tmpPots = pots;
  for (let i = 0; i < relevantRules.length; i++) {
    const rule = relevantRules[i];
    let position = pots.indexOf(rule[0]);
    while (position !== -1) {
      tmpPots = tmpPots.substring(0, position + 2) + rule[1] + tmpPots.substring(position + 3);
      position = pots.indexOf(rule[0], position + 1);
    }
  }
  pots = tmpPots;
  console.log(String(iteration).padStart(3, '0'), pots.substr(0, 182));
}

const potSum = pots.split('').reduce((total, pot, index) => {
  if (pot === '#') {
    return total += (index - OFFSET + ITERATIONS_DIFF);
  }
  return total;
}, 0)

console.log('SUM: ', potSum);
