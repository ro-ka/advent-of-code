use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<String>) {
    println!("Part 1: {:?}.", entries);
}

fn part2(entries: &Vec<String>) {
    println!("Part 2: {:?}.", entries);
}
