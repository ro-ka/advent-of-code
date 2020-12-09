use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();

    println!(
        "Part 1: {} trees hit.",
        get_hit_trees_for_slope(&entries, &(3, 1))
    );
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    println!(
        "Part 2: {}.",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|s| get_hit_trees_for_slope(&entries, s))
            .fold(1, |acc, x| acc * x)
    );
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn get_hit_trees_for_slope(entries: &Vec<String>, diff: &(usize, usize)) -> usize {
    entries
        .iter()
        .step_by(diff.1)
        .zip((0..entries.len() * diff.0).step_by(diff.0))
        .filter(|&(row, x)| row.chars().cycle().nth(x).unwrap() == '#')
        .count()
}
