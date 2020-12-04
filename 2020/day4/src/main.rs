use regex::Regex;

fn main() {
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
    part2(&passports_with_required_fields);
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
                "byr" => match rule[1].parse::<usize>() {
                    Ok(year) => year >= 1920 && year <= 2002,
                    Err(_) => false,
                },
                "iyr" => match rule[1].parse::<usize>() {
                    Ok(year) => year >= 2010 && year <= 2020,
                    Err(_) => false,
                },
                "eyr" => match rule[1].parse::<usize>() {
                    Ok(year) => year >= 2020 && year <= 2030,
                    Err(_) => false,
                },
                "hgt" => match hgt_re.captures(&rule[1]) {
                    Some(cap) => match cap[1].parse::<usize>() {
                        Ok(height) => match &cap[2] {
                            "cm" => height >= 150 && height <= 193,
                            "in" => height >= 59 && height <= 76,
                            _ => false,
                        },
                        Err(_) => false,
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
