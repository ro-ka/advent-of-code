interface Monkey {
  id: number;
  items: number[];
  operation: {
    operator: string;
    value: string;
  }
  test: {
    value: number;
    yes: number;
    no: number;
  }
}

function parse(input: string) {
  return input.split('\n\n')
    .map(raw => raw.split('\n'))
    .map(raw => {
      const id = Number(raw[0].replace('Monkey ', '').replace(':', ''));
      const items = raw[1].replace('  Starting items: ', '').split(', ').map(Number);
      const operationRaw = raw[2].replace('  Operation: new = old ', '');
      const operation = {
        operator: operationRaw.substring(0, 1),
        value: operationRaw.substring(2),
      };
      const test = {
        value: Number(raw[3].replace('  Test: divisible by ', '')),
        yes: Number(raw[4].replace('    If true: throw to monkey ', '')),
        no: Number(raw[5].replace('    If false: throw to monkey ', '')),
      };

      return { id, items, operation, test } as Monkey;
    });
} 

function getTargetMonkey(oldWorryLevel: number, monkey: Monkey, divide: boolean): {
  targetMonkey: number,
  newWorryLevel: number
} {
  const secondOperator = monkey.operation.value === 'old' ? oldWorryLevel : Number(monkey.operation.value);
  let newWorryLevel = 0;

  if (monkey.operation.operator === '*') {
    newWorryLevel = oldWorryLevel * secondOperator;
  } else {
    newWorryLevel = oldWorryLevel + secondOperator;
  }

  if (divide) {
    newWorryLevel = Math.floor(newWorryLevel / 3);
  }

  const targetMonkey = (newWorryLevel % monkey.test.value === 0)
    ? monkey.test.yes
    : monkey.test.no;

  return {targetMonkey, newWorryLevel};
}

export function part1(input: string) {
  const monkeys = [...parse(input)];
  const monkeyActivity = new Map(monkeys.map(monkey => ([monkey.id, 0])));

  for (let i = 0; i < 20; i++) {
    monkeys.forEach(monkey => {
      while (monkey.items.length) {
        monkeyActivity.set(monkey.id, monkeyActivity.get(monkey.id) + 1);
        const value = monkey.items.shift();
        const {targetMonkey, newWorryLevel} = getTargetMonkey(value, monkey, true);
        monkeys[targetMonkey].items.push(newWorryLevel);
      }
    });
  }

  const mostActive = Array.from(monkeyActivity.values()).sort((a, b) => b - a);
  return mostActive[0] * mostActive[1];
}

export function part2(input: string) {
  const monkeys = [...parse(input)];
  const monkeyActivity = new Map(monkeys.map(monkey => ([monkey.id, 0])));

  for (let i = 0; i < 10000; i++) {
    monkeys.forEach(monkey => {
      while (monkey.items.length) {
        monkeyActivity.set(monkey.id, monkeyActivity.get(monkey.id) + 1);
        const value = monkey.items.shift();
        const {targetMonkey, newWorryLevel} = getTargetMonkey(value, monkey, false);
        monkeys[targetMonkey].items.push(newWorryLevel);
      }
    });
  }

  const mostActive = Array.from(monkeyActivity.values()).sort((a, b) => b - a);
  return mostActive[0] * mostActive[1];
}