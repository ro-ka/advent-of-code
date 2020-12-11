use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let lines = helper::read_file_strings().unwrap();
    let entries: Vec<Vec<char>> = lines.iter().map(|e| e.trim().chars().collect()).collect();

    part1(entries.clone());
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(entries.clone());
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: Vec<Vec<char>>) {
    let mut iterated_entries_before = entries.clone();
    let mut iterated_entries = iterate_seating_part(&iterated_entries_before, 1);

    while did_change(&iterated_entries_before, &iterated_entries) {
        iterated_entries_before = iterated_entries.clone();
        iterated_entries = iterate_seating_part(&iterated_entries, 1);
    }

    println!("Part 1: {:?}.", count_seats(&iterated_entries));
}

fn iterate_seating_part(entries: &Vec<Vec<char>>, distance: usize) -> Vec<Vec<char>> {
    let rows = entries.len();
    let columns = entries[0].len();
    let mut iterated_entries: Vec<Vec<char>> = vec![vec!['.'; columns]; rows];

    for r in 0..rows {
        for c in 0..columns {
            let r_start = if r > 0 { r - 1 } else { 0 };
            let r_end = if r < rows - 1 { r + 1 } else { rows - 1 };

            let c_start = if c > 0 { c - 1 } else { 0 };
            let c_end = if c < columns - 1 { c + 1 } else { columns - 1 };

            let adjactent_seats = &entries[r_start..=r_end]
                .iter()
                .map(|r| &r[c_start..=c_end])
                .flatten()
                .filter(|v| **v == '#')
                .count()
                - if entries[r][c] == '#' { 1 } else { 0 };

            iterated_entries[r][c] = match entries[r][c] {
                'L' => match adjactent_seats == 0 {
                    true => '#',
                    false => 'L',
                },
                '#' => match adjactent_seats >= 4 {
                    true => 'L',
                    false => '#',
                },
                _ => '.',
            }
        }
    }

    iterated_entries
}

fn did_change(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    a.iter().flatten().collect::<String>() != b.iter().flatten().collect::<String>()
}

fn count_seats(entries: &Vec<Vec<char>>) -> usize {
    entries.iter().flatten().filter(|v| **v == '#').count()
}

fn part2(entries: Vec<Vec<char>>) {
    println!("Part 2: {:?}.", entries);
}
