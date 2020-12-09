use itertools::Itertools;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_i64s().unwrap();

    let invalid_number = part1(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&entries, invalid_number);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<i64>) -> i64 {
    let preamble = 25;

    let invalid_number = (&entries[..])
        .windows(preamble + 1)
        .find(|window| {
            window[0..preamble]
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != window[preamble])
        })
        .map(|window| window[preamble])
        .unwrap();

    println!("Part 1: {:?}.", invalid_number);
    invalid_number
}

fn part2(entries: &Vec<i64>, invalid_number: i64) {
    let (mut i, mut j, mut sum) = (0, 0, 0);

    while sum != invalid_number {
        if sum < invalid_number {
            sum += entries[j];
            j += 1;
        } else {
            sum -= entries[i];
            i += 1;
        }
    }

    let max = entries[i..j].iter().max().unwrap();
    let min = entries[i..j].iter().min().unwrap();
    println!("Part 2: {:?}.", max + min);
}
