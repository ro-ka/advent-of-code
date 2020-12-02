use regex::Regex;

struct Entry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn main() {
    let raw_entries = helper::read_file_strings().unwrap();

    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$").unwrap();

    let entries: Vec<Entry> = raw_entries
        .iter()
        .map(|entry| {
            let cap = re.captures(entry).unwrap();
            return Entry {
                min: cap[1].parse::<usize>().unwrap(),
                max: cap[2].parse::<usize>().unwrap(),
                letter: cap[3].to_string().chars().nth(0).unwrap(),
                password: cap[4].to_string(),
            };
        })
        .collect();

    part1(&entries);
    part2(&entries);
}

fn part1(entries: &Vec<Entry>) {
    let valid_passwords = entries
        .iter()
        .filter(|entry| {
            let letter_count = entry
                .password
                .chars()
                .filter(|c| c == &entry.letter)
                .count();
            return letter_count >= entry.min && letter_count <= entry.max;
        })
        .count();

    println!("Part 1: {} valid passwords.", valid_passwords);
}

fn part2(entries: &Vec<Entry>) {
    let valid_passwords = entries
        .iter()
        .filter(|entry| {
            entry.password.chars().nth(entry.min) == Some(entry.letter)
                || entry.password.chars().nth(entry.max) == Some(entry.letter)
        })
        .count();

    println!("Part 2: {} valid passwords.", valid_passwords);
}
