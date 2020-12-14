use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&entries);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<String>) {
    let mut mask: Vec<char> = "".chars().collect();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for entry in entries {
        if entry.starts_with("mask = ") {
            mask = entry.trim_start_matches("mask = ").chars().collect();
        } else if entry.starts_with("mem") {
            let (mem_address, raw_value) = decode_entry(entry);
            let raw_bin_value: Vec<char> = format!("{:036b}", raw_value).chars().collect();
            let bin_value = (0..mask.len())
                .into_iter()
                .map(|i| match mask[i] {
                    'X' => raw_bin_value[i],
                    _ => mask[i],
                })
                .collect::<String>();

            let int_value = usize::from_str_radix(&bin_value, 2).unwrap();

            memory.insert(mem_address, int_value);
        }
    }

    println!(
        "Part 1: {:?}.",
        memory.values().into_iter().fold(0, |acc, v| acc + v)
    );
}

fn part2(entries: &Vec<String>) {
    let mut mask: Vec<char> = "".chars().collect();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for entry in entries {
        if entry.starts_with("mask = ") {
            mask = entry.trim_start_matches("mask = ").chars().collect();
        } else if entry.starts_with("mem") {
            let (raw_mem_address, value) = decode_entry(entry);
            let raw_bin_mem_address: Vec<char> =
                format!("{:036b}", raw_mem_address).chars().collect();

            let bin_mem_address = (0..mask.len())
                .into_iter()
                .map(|i| match mask[i] {
                    '0' => raw_bin_mem_address[i],
                    _ => mask[i],
                })
                .collect::<Vec<char>>();

            let mut options = vec![bin_mem_address.clone()];

            for i in 0..bin_mem_address.iter().count() {
                if bin_mem_address[i] == 'X' {
                    for k in 0..options.len() {
                        let mut clone = options[k].clone();
                        clone[i] = '0';
                        options.push(clone);
                        options[k][i] = '1';
                    }
                }
            }

            for option in options {
                let int_mem_address =
                    usize::from_str_radix(&option.iter().collect::<String>(), 2).unwrap();
                memory.insert(int_mem_address, value);
            }
        }
    }

    println!(
        "Part 2: {:?}.",
        memory.values().into_iter().fold(0, |acc, v| acc + v)
    );
}

fn decode_entry(entry: &str) -> (usize, usize) {
    let re = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)$").unwrap();
    let cap = re.captures(entry).unwrap();
    let mem_address = cap[1].parse::<usize>().unwrap();
    let raw_value = cap[2].parse::<usize>().unwrap();

    (mem_address, raw_value)
}
