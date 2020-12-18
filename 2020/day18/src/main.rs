use regex::Regex;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();

    println!("Part 1: {:?}.", get_sum(&entries, &calc_group_1));
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    println!("Part 2: {:?}.", get_sum(&entries, &calc_group_2));
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn get_sum(entries: &Vec<String>, calc_group: &dyn Fn(&String) -> usize) -> usize {
    let group_re = Regex::new(r"\((\d+)(?: ([\+\*]\s\d+))+\)").unwrap();

    entries
        .iter()
        .map(|l| {
            let mut line = l.clone();

            while let Some(captures) = group_re.captures(&line) {
                let group = captures[0].to_string();
                line = line.replace(&group, &calc_group(&group).to_string());
            }

            calc_group(&line)
        })
        .sum::<usize>()
}

fn calc_group_1(raw_input: &String) -> usize {
    let re = Regex::new(r"(\d+)\s([\+|\*])\s(\d+)").unwrap();

    let mut input = raw_input.clone();
    input = input.trim_start_matches("(").to_string();
    input = input.trim_end_matches(")").to_string();

    while let Some(captures) = re.captures(&input) {
        let exp = captures[0].to_string();
        let op = captures[2].to_string();
        let a: usize = captures[1].parse().unwrap();
        let b: usize = captures[3].parse().unwrap();

        if op == "*" {
            input = input.replacen(&exp, &(a * b).to_string(), 1);
        } else if op == "+" {
            input = input.replacen(&exp, &(a + b).to_string(), 1);
        }
    }

    input.parse().unwrap()
}

fn calc_group_2(raw_input: &String) -> usize {
    let add_re = Regex::new(r"(\d+)\s\+\s(\d+)").unwrap();
    let prod_re = Regex::new(r"(\d+)\s\*\s(\d+)").unwrap();

    let mut input = raw_input.clone();
    input = input.trim_start_matches("(").to_string();
    input = input.trim_end_matches(")").to_string();

    while let Some(captures) = add_re.captures(&input) {
        let exp = captures[0].to_string();
        let a: usize = captures[1].parse().unwrap();
        let b: usize = captures[2].parse().unwrap();

        input = input.replacen(&exp, &(a + b).to_string(), 1);
    }

    while let Some(captures) = prod_re.captures(&input) {
        let exp = captures[0].to_string();
        let a: usize = captures[1].parse().unwrap();
        let b: usize = captures[2].parse().unwrap();

        input = input.replacen(&exp, &(a * b).to_string(), 1);
    }

    input.parse().unwrap()
}
