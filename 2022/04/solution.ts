export function part1(input: string) {
  return input
    .split('\n')
    .map(row => row
      .split(',')
      .map(section => ({
        min: Number(section.split('-')[0]),
        max: Number(section.split('-')[1]),
      }))
    )
    .filter(([section1, section2]) => {
      if (
        section1.min >= section2.min &&
        section1.min <= section2.max &&
        section1.max >= section2.min &&
        section1.max <= section2.max
      ) {
        return true;
      }

      if (
        section2.min >= section1.min &&
        section2.min <= section1.max &&
        section2.max >= section1.min &&
        section2.max <= section1.max
      ) {
        return true;
      }

      return false;
    })
    .length;
}

export function part2(input: string) {  return input
    .split('\n')
    .map(row => row
      .split(',')
      .map(section => ({
        min: Number(section.split('-')[0]),
        max: Number(section.split('-')[1]),
      }))
    )
    .filter(([section1, section2]) => {
      if (
        section1.min <= section2.max &&
        section1.max >= section2.min
      ) {
        return true;
      }

      if (
        section2.min <= section1.max &&
        section2.max >= section1.min
      ) {
        return true;
      }

      return false;
    })
    .length;

}