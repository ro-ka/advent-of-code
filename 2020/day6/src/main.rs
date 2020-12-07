use std::collections::HashMap;

fn main() {
    let data = helper::read_file_to_string().unwrap();

    let groups_data: Vec<(usize, HashMap<char, usize>)> = data
        .trim()
        .split("\n\n")
        .map(|group| {
            let mut letters: HashMap<char, usize> = HashMap::new();
            let people = group.split("\n").collect::<Vec<&str>>().len();

            for c in group.replace("\n", "").chars() {
                let count = letters.entry(c).or_insert(0);
                *count += 1;
            }

            (people, letters)
        })
        .collect();

    part1(&groups_data);
    part2(&groups_data);
}

fn part1(groups_data: &Vec<(usize, HashMap<char, usize>)>) {
    let count = groups_data
        .iter()
        .map(|(_, letters)| letters.len())
        .fold(0, |acc, x| acc + x);

    println!("Part 1: {:?}.", count);
}

fn part2(groups_data: &Vec<(usize, HashMap<char, usize>)>) {
    let count = groups_data
        .iter()
        .map(|(people, letters)| letters.iter().filter(|(_, c)| *c == people).count())
        .fold(0, |acc, x| acc + x);

    println!("Part 2: {:?}.", count);
}
