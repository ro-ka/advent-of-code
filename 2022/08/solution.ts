function isVisible(forest: number[][], x: number, y: number): boolean {
  const tree = forest[y][x];
  const column = forest.map(c => c[x]);

  const visibleLeft = !forest[y].slice(0, x).some(t => t >= tree);
  const visibleRight = !forest[y].slice(x + 1, forest[y].length).some(t => t >= tree);
  const visibleTop = !column.slice(0, y).some(t => t >= tree);
  const visibleBottom = !column.slice(y + 1, column[column.length]).some(t => t >= tree);

  return visibleLeft || visibleRight || visibleBottom || visibleTop;
}

export function part1(input: string) {
  const forest = input.split('\n').map(row => row.split('').map(Number));

  let visibleTrees = forest.length * 2 + (forest[0].length - 2) * 2;

  for (let y = 1; y < forest.length - 1; y++) {
    for (let x = 1; x < forest.length - 1; x++) {
      if (isVisible(forest, x, y)) {
        visibleTrees++;
      }
    }
  }

  return visibleTrees;
}

function getViewDistance(viewDirection: number[], tree: number): number {
  return (viewDirection.findIndex(t => t >= tree) + 1) || viewDirection.length;
}

function getScore(forest: number[][], x: number, y: number): number {
  const tree = forest[y][x];
  const column = forest.map(c => c[x]);

  const distanceLeft = getViewDistance([...forest[y].slice(0, x)].reverse(), tree);
  const distanceRight = getViewDistance([...forest[y].slice(x + 1, forest[y].length)], tree);
  const distanceTop = getViewDistance([...column.slice(0, y)].reverse(), tree);
  const distanceBottom = getViewDistance([...column.slice(y + 1, column.length)], tree);

  return distanceLeft * distanceTop * distanceBottom * distanceRight;
}

export function part2(input: string) {
  const forest = input.split('\n').map(row => row.split('').map(Number));
  
  let score = 0;

  for (let y = 1; y < forest.length - 1; y++) {
    for (let x = 1; x < forest.length - 1; x++) {
      score = Math.max(score, getScore(forest, x, y));
    }
  }

  return score;
}