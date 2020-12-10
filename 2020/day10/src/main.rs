use std::cmp::min;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let mut entries = helper::read_file_usizes().unwrap();
    entries.sort();

    entries.insert(0, 0);
    let max = entries.iter().max().unwrap() + 3;
    entries.push(max);

    part1(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<usize>) {
    let (mut diff1, mut diff3) = (0, 0);

    for i in 1..entries.len() {
        match entries[i] - entries[i - 1] {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => unimplemented!(),
        }
    }

    println!("Part 1: {:?} * {:?} = {:?}", diff1, diff3, diff1 * diff3);
}

fn part2(entries: &Vec<usize>) {
    let mut variants_per_number = vec![1 as u64; *entries.last().unwrap() + 1];

    for i in (0..entries.len() - 1).rev() {
        variants_per_number[entries[i]] = entries[i + 1..min(i + 4, entries.len())]
            .iter()
            .filter(|e| *e - entries[i] <= 3)
            .fold(0, |acc, e| acc + variants_per_number[*e] as u64);
    }

    println!("Part 2: {:?}.", variants_per_number[0]);
}
