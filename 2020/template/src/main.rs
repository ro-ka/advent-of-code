fn main() {
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    part2(&entries);
}

fn part1(entries: &Vec<String>) {
    println!("Part 1: {:?}.", entries);
}

fn part2(entries: &Vec<String>) {
    println!("Part 2: {:?}.", entries);
}
