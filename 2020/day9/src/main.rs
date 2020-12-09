fn main() {
    let entries = helper::read_file_i64s().unwrap();

    let invalid_numer_index = part1(&entries);
    part2(&entries, invalid_numer_index);
}

fn part1(entries: &Vec<i64>) -> usize {
    let preamble = 25;

    for index in preamble..entries.len() {
        if !is_sum_of_preamble(entries, preamble, index) {
            println!("Part 1: {:?}.", entries[index]);
            return index;
        }
    }

    0
}

fn part2(entries: &Vec<i64>, invalid_number_index: usize) {
    let invalid_number = entries[invalid_number_index];
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

fn is_sum_of_preamble(entries: &Vec<i64>, preamble: usize, index: usize) -> bool {
    let number = entries[index];
    for i in index - preamble..index {
        for k in index - preamble..index {
            if entries[i] + entries[k] == number {
                return true;
            }
        }
    }

    false
}
