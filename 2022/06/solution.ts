function getIndex(input: string, markerLength: number) {
  let index = markerLength;
  let allDifferent = false;

  while (!allDifferent) {
    const duplicatesRemoved = [...new Set(input.substring(index - markerLength, index).split(''))];

    if (duplicatesRemoved.length === markerLength) {
      allDifferent = true;
    } else {
      index++;
    }
  }

  return index;  
}

export function part1(input: string) {
  return getIndex(input, 4);
}

export function part2(input: string) {
  return getIndex(input, 14);
}