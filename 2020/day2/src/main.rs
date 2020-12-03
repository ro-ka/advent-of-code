use regex::Regex;

fn main() {
    let raw_entries = helper::read_file_strings().unwrap();

    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$").unwrap();

    let entries: Vec<(usize, usize, char, String)> = raw_entries
        .iter()
        .map(|entry| {
            let cap = re.captures(entry).unwrap();
            (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
                cap[3].to_string().chars().nth(0).unwrap(),
                cap[4].to_string(),
            )
        })
        .collect();

    part1(&entries);
    part2(&entries);
}

fn part1(entries: &Vec<(usize, usize, char, String)>) {
    let valid_passwords = entries
        .iter()
        .map(|(min, max, letter, password)| {
            (min, max, password.chars().filter(|c| c == letter).count())
        })
        .filter(|(min, max, letter_count)| letter_count >= min && letter_count <= max)
        .count();

    println!("Part 1: {} valid passwords.", valid_passwords);
}

fn part2(entries: &Vec<(usize, usize, char, String)>) {
    let valid_passwords = entries
        .iter()
        .filter(|(min, max, letter, password)| {
            password.chars().nth(*min) == Some(*letter)
                || password.chars().nth(*max) == Some(*letter)
        })
        .count();

    println!("Part 2: {} valid passwords.", valid_passwords);
}
