use regex::Regex;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let data = helper::read_file_to_string().unwrap();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports_with_required_fields: Vec<Vec<Vec<String>>> = data
        .split("\n\n")
        .map(|p| {
            p.replace("\n", " ")
                .split_whitespace()
                .map(|r| r.split(":").map(|v| v.to_string()).collect())
                .collect::<Vec<Vec<String>>>()
        })
        .filter(|rules| {
            required_fields
                .iter()
                .all(|req| match rules.iter().find(|r| r[0] == *req) {
                    Some(_) => true,
                    None => false,
                })
        })
        .collect();

    part1(&passports_with_required_fields);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();
    part2(&passports_with_required_fields);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(passports: &Vec<Vec<Vec<String>>>) {
    println!("Part 1: {}.", passports.len());
}

fn part2(passports: &Vec<Vec<Vec<String>>>) {
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hcl_re = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    let valid_passports: Vec<&Vec<Vec<String>>> = passports
        .iter()
        .filter(|rules| {
            rules.iter().all(|rule| match rule[0].as_str() {
                "byr" => validate_range(&rule[1], 1920, 2002),
                "iyr" => validate_range(&rule[1], 2010, 2020),
                "eyr" => validate_range(&rule[1], 2020, 2030),
                "hgt" => match hgt_re.captures(&rule[1]) {
                    Some(cap) => match &cap[2] {
                        "cm" => validate_range(&cap[1].to_string(), 150, 193),
                        "in" => validate_range(&cap[1].to_string(), 59, 76),
                        _ => false,
                    },
                    None => false,
                },
                "hcl" => hcl_re.is_match(&rule[1]),
                "ecl" => ecl_re.is_match(&rule[1]),
                "pid" => pid_re.is_match(&rule[1]),
                _ => true,
            })
        })
        .collect();
    println!("Part 2: {}.", valid_passports.len());
}

fn validate_range(value: &String, min: usize, max: usize) -> bool {
    match value.parse::<usize>() {
        Ok(number) => number >= min && number <= max,
        Err(_) => false,
    }
}
