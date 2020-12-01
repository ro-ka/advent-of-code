const INITIAL = '#...##.#...#..#.#####.##.#..###.#.#.###....#...#...####.#....##..##..#..#..#..#.#..##.####.#.#.###';

const rules = [{
    pattern: '.....',
    new: '.'
  },
  {
    pattern: '..#..',
    new: '#'
  },
  {
    pattern: '..##.',
    new: '#'
  },
  {
    pattern: '#..##',
    new: '.'
  },
  {
    pattern: '..#.#',
    new: '#'
  },
  {
    pattern: '####.',
    new: '.'
  },
  {
    pattern: '##.##',
    new: '.'
  },
  {
    pattern: '#....',
    new: '.'
  },
  {
    pattern: '###..',
    new: '#'
  },
  {
    pattern: '#####',
    new: '#'
  },
  {
    pattern: '##..#',
    new: '#'
  },
  {
    pattern: '#.###',
    new: '#'
  },
  {
    pattern: '#..#.',
    new: '#'
  },
  {
    pattern: '.####',
    new: '#'
  },
  {
    pattern: '#.#..',
    new: '#'
  },
  {
    pattern: '.###.',
    new: '#'
  },
  {
    pattern: '.##..',
    new: '#'
  },
  {
    pattern: '.#...',
    new: '#'
  },
  {
    pattern: '.#.##',
    new: '#'
  },
  {
    pattern: '##...',
    new: '#'
  },
  {
    pattern: '..###',
    new: '.'
  },
  {
    pattern: '##.#.',
    new: '.'
  },
  {
    pattern: '...##',
    new: '.'
  },
  {
    pattern: '....#',
    new: '.'
  },
  {
    pattern: '###.#',
    new: '.'
  },
  {
    pattern: '#.##.',
    new: '#'
  },
  {
    pattern: '.##.#',
    new: '.'
  },
  {
    pattern: '.#..#',
    new: '#'
  },
  {
    pattern: '#.#.#',
    new: '#'
  },
  {
    pattern: '.#.#.',
    new: '#'
  },
  {
    pattern: '...#.',
    new: '#'
  },
  {
    pattern: '#...#',
    new: '#'
  }
];
const relevantRules = rules.filter(rule => rule.pattern[2] !== rule.new);

const startTime = new Date();

const offset = 40;
let pots = `........................................${INITIAL}........................................`;

for (let iteration = 0; iteration < 20; iteration++) {
  let tmpPots = pots;
  for (let index = 2; index < pots.length - 2; index++) {
    const potArea = pots.substr(index - 2, 5);
    relevantRules.forEach(rule => {
      if (potArea === rule.pattern) {
        tmpPots = tmpPots.substring(0, index) + rule.new + tmpPots.substring(index + 1);
      }
    });
  }
  pots = tmpPots;
}

const potSum = pots.split('').reduce((total, pot, index) => {
  if (pot === '#') {
    return total += (index - offset);
  }
  return total;
}, 0)

const endTime = new Date();

console.log(endTime - startTime);
console.log(potSum);
