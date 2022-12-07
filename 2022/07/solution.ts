function buildTree(input: string) {
  let currentDirectory = [];
  const tree = new Map();

  input.split('\n').forEach(line => {
    if (line.startsWith('$ ')) {
      const command = line.substring(2);

      if (command === 'cd /') {
        currentDirectory = [];
      } else if (command === 'cd ..') {
        currentDirectory.pop();
      } else if (command.startsWith('cd ')) {
        currentDirectory.push(command.substring(3));
      }
    } else {
      const fileMatch = line.match(/(\d+)\s([\w\.]+)/);
      if (fileMatch) {
        const [_, fileSize, fileName] = fileMatch;
        tree.set([...currentDirectory, fileName], Number(fileSize));
      }
    }
  });

  return tree;
}

function getDirectorySizes(tree: Map<string[], number>) {
  const directorySizes: {[key: string]: number} = {};

  tree.forEach((size, filePath) => {
    let path = [...filePath].slice(0, -1);

    while (path.length > 0) {
      const pathString = path.join('/');
      const oldSize = directorySizes[pathString] || 0;
      directorySizes[pathString] = oldSize + size;
      path = [...path].slice(0, -1);
    }
  });

  return directorySizes;
}

export function part1(input: string) {
  const tree = buildTree(input);
  const directorySizes = getDirectorySizes(tree);

  return Object.values(directorySizes)
    .filter(size => size <= 100000)
    .reduce((sum, value) => sum + value, 0);
}

export function part2(input: string) {
  const tree = buildTree(input);
  const directorySizes = getDirectorySizes(tree);
  const totalSpace = 70000000;
  const spaceNeeded = 30000000;
  let spaceUsed = 0;
  tree.forEach((size) => {
    spaceUsed += size;
  });
  const freeSpace = totalSpace - spaceUsed;
  const spaceToFree = spaceNeeded - freeSpace;

  const possibleToDelete = Object.entries(directorySizes)
    .filter(([path, size]) => size >= spaceToFree)
    .sort(([pathA, sizeA], [pathB, sizeB]) => sizeA - sizeB);

  return possibleToDelete[0][1];
}