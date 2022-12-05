function buildStacks(input: string): {[key: string]: string[]} {
  const stackInput = input.split('\n\n')[0].split('\n').reverse();
  const stackCountRow = stackInput.shift();
  const stacks = {};
  
  stackCountRow
    .trim()
    .split('  ')
    .forEach(stackNumber => stacks[Number(stackNumber)] = []);
  
  stackInput.forEach(row => {
    Object.keys(stacks).forEach(stackNumber => {
      const cargo = row[(Number(stackNumber) - 1) * 4 + 1];
      if (cargo !== ' ') {
        stacks[stackNumber].push(cargo);
      }
    });
  });

  return stacks;
}

function buildMoves(input: string) {
  return input
    .split('\n\n')
    [1]
    .split('\n')
    .map(row => {
      const matches = row.match(/move (\d+) from (\d+) to (\d+)/);
      return {amount: Number(matches[1]), from: Number(matches[2]), to: Number(matches[3])};
    });
}

export function part1(input: string) {
  const stacks = {...buildStacks(input)};
  const moves = buildMoves(input);

  moves.forEach(({amount, from, to}) => {
    let i = 0;
    while (i < amount) {
      const toMove = stacks[from].pop();
      stacks[to].push(toMove);
      i++;
    }
  });

  return Object.values(stacks).map(stack => stack[stack.length -1]).join('');
}

export function part2(input: string) {
  const stacks = {...buildStacks(input)};
  const moves = buildMoves(input);

  moves.forEach(({amount, from, to}) => {
    let i = 0;
    const toMove = [];
    while (i < amount) {
      toMove.unshift(stacks[from].pop());
      i++;
    }

    stacks[to] = [...stacks[to], ...toMove];
  });

  return Object.values(stacks).map(stack => stack[stack.length -1]).join('');
}