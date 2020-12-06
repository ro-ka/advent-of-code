fn main() {
    let entries = helper::read_file_strings().unwrap();

    let mut seat_ids = entries.iter().map(|l| {
        let rows: Vec<usize> = (0..=127).collect();
        let columns: Vec<usize> = (0..=7).collect();
        let (row_rules, column_rules) = l.split_at(7);
        let row = find_value(&rows[..], &row_rules);
        let column = find_value(&columns[..], &column_rules);
        row * 8 + column
    }).collect();

    part1(&seat_ids);
    part2(&mut seat_ids);
}

fn part1(seat_ids: &Vec<usize>) {
    let max_seat_id = seat_ids.iter().max();
    println!("Part 1: {:?}.", max_seat_id);
}

fn part2(seat_ids: &mut Vec<usize>) {
    seat_ids.sort();
    let mut seat_id_before = seat_ids[0];
    for seat_id in seat_ids {
        if *seat_id - seat_id_before == 2 {
            println!("Part 2: {:?}.", *seat_id - 1);
        }
        seat_id_before = *seat_id;
    }
}

fn find_value(numbers: &[usize], rules: &str)-> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let (start, end) = numbers.split_at(numbers.len() / 2);
    let (current_rule, rest_rules) = rules.split_at(1);

    match current_rule {
        "B" => find_value(end, rest_rules),
        "R" => find_value(end, rest_rules),
        _ => find_value(start, rest_rules)
    }
}
