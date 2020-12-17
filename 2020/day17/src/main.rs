use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    // part2(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<String>) {
    let mut state: Vec<(i32, i32, i32)> = vec![];

    entries.iter().enumerate().for_each(|(x, row)| {
        row.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                state.push((x as i32, y as i32, 0i32));
            }
        });
    });

    for _ in 0..6 {
        state = cycle(state.clone());
    }

    println!("Part 1: {:?}.", state.len());
}

// fn part2(entries: &Vec<String>) {
//     println!("Part 2: {:?}.", entries);
// }

fn cycle(state: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut new_state: Vec<(i32, i32, i32)> = vec![];

    let x_values = state.iter().map(|(x, _, _)| x);
    let y_values = state.iter().map(|(_, y, _)| y);
    let z_values = state.iter().map(|(_, _, z)| z);

    for x in (x_values.clone().min().unwrap() - 1)..=(x_values.clone().max().unwrap() + 1) {
        for y in (y_values.clone().min().unwrap() - 1)..=(y_values.clone().max().unwrap() + 1) {
            for z in (z_values.clone().min().unwrap() - 1)..=(z_values.clone().max().unwrap() + 1) {
                let key = (x, y, z);
                let currently_active = state.iter().find(|k| **k == key).is_some();
                let active_neighbors = state
                    .iter()
                    .map(|k| *k)
                    .filter(|k| {
                        *k != key
                            && (k.0 - key.0).abs() <= 1
                            && (k.1 - key.1).abs() <= 1
                            && (k.2 - key.2).abs() <= 1
                    })
                    .count();

                if active_neighbors == 3 || (active_neighbors == 2 && currently_active) {
                    new_state.push(key);
                }
            }
        }
    }

    new_state
}
