export function part1(input: string) {
  return input
    .split('\n')
    .map(rucksack => {
      let letter: string = null;
      rucksack.substring(0, rucksack.length / 2).split('').forEach(l => {
        if (rucksack.substring(rucksack.length / 2).includes(l)) {
          letter = l;
        }
      })

      return letter;
    })
    .map(letter => letter.charCodeAt(0))
    .map(value => value > 95 ? value - 96 : value - 38)
    .reduce((sum, value) => sum + value, 0);
}

export function part2(input: string) {
  return input
    .split('\n')
    .reduce((groups, rucksack) => {
      const group = groups.pop();

      if (group && group.length < 3) {
        return  [...groups, [...group, rucksack]];
      }

      return [...groups, group, [rucksack]];
    }, [])
    .filter(Boolean)
    .map(group => {
      let letter: string = null;
      group[0].split('').forEach(l => {
        if (group[1].includes(l) && group[2].includes(l)) {
          letter = l;
        }
      });

      return letter;
    })
    .map(letter => letter.charCodeAt(0))
    .map(value => value > 95 ? value - 96 : value - 38)
    .reduce((sum, value) => sum + value, 0);
}