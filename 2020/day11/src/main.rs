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
    println!("Part 1: {:?}.", get_occupied_seats(entries, 4, 1));
}

fn part2(entries: Vec<Vec<char>>) {
    println!("Part 2: {:?}.", get_occupied_seats(entries, 5, 100000));
}

fn get_occupied_seats(entries: Vec<Vec<char>>, to_occupied: usize, distance: usize) -> usize {
    let mut iterated_entries_before = entries.clone();
    let mut iterated_entries = iterate_seating(&iterated_entries_before, to_occupied, distance);

    while did_change(&iterated_entries_before, &iterated_entries) {
        iterated_entries_before = iterated_entries.clone();
        iterated_entries = iterate_seating(&iterated_entries, to_occupied, distance);
    }

    iterated_entries.iter().flatten().filter(|v| **v == '#').count()
}

fn iterate_seating(entries: &Vec<Vec<char>>, to_occupied: usize, distance: usize) -> Vec<Vec<char>> {
    let rows = entries.len();
    let columns = entries[0].len();
    let mut iterated_entries: Vec<Vec<char>> = vec![vec!['.'; columns]; rows];

    for r in 0..rows {
        for c in 0..columns {
            let adjacent_seats = vec![
              get_first_seat_in_direction(&entries, r, c, 1, 1, distance),
              get_first_seat_in_direction(&entries, r, c, 1, 0, distance),
              get_first_seat_in_direction(&entries, r, c, 1, -1, distance),
              get_first_seat_in_direction(&entries, r, c, 0, 1, distance),
              get_first_seat_in_direction(&entries, r, c, 0, -1, distance),
              get_first_seat_in_direction(&entries, r, c, -1, 1, distance),
              get_first_seat_in_direction(&entries, r, c, -1, 0, distance),
              get_first_seat_in_direction(&entries, r, c, -1, -1, distance),
            ];
            let occupied_seats = adjacent_seats.iter().filter(|s| **s == '#').count();

            iterated_entries[r][c] = match entries[r][c] {
                'L' => match occupied_seats == 0 {
                    true => '#',
                    false => 'L',
                },
                '#' => match occupied_seats >= to_occupied {
                    true => 'L',
                    false => '#',
                },
                _ => '.',
            }
        }
    }

    iterated_entries
}

fn get_first_seat_in_direction(entries: &Vec<Vec<char>>, row: usize, column: usize, row_diff: i8, column_diff: i8, distance: usize) -> char {
  let (mut r, mut c, mut d) = (row + row_diff as usize, column + column_diff as usize, 1);

  while r < entries.len() && c < entries[0].len() && d <= distance {
      let entry = entries[r][c];
      if entry == 'L' || entry == '#' {
        return entry;
      }
      r += row_diff as usize;
      c += column_diff as usize;
      d += 1;
  }

  '.'
}

fn did_change(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    a.iter().flatten().collect::<String>() != b.iter().flatten().collect::<String>()
}
