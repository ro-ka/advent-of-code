use regex::Regex;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let raw_data = helper::read_file_to_string().unwrap();
    let data: Vec<&str> = raw_data.split("\n\n").collect();
    let entries: Vec<&str> = data[1].lines().collect();

    let mut rules: Vec<String> = vec!["".to_string(); data[0].lines().count()];
    for l in data[0].lines() {
        let parts: Vec<&str> = l.split(": ").collect();
        rules[parts[0].parse::<usize>().unwrap()] = parts[1].replace("\"", "");
    }

    let index_re = Regex::new(r"(\d+)").unwrap();

    let mut changed = true;
    while changed {
        changed = false;

        rules = rules
            .iter()
            .map(|r| match index_re.captures(&r) {
                Some(cap) => {
                    let index = cap[1].to_string();
                    let mut rule = rules[index.parse::<usize>().unwrap()].to_string();
                    if rule.len() > 1 {
                        rule.insert(0, '(');
                        rule.push(')');
                    }
                    changed = true;
                    r.replace(&index, &rule).to_string()
                }
                None => r.to_string(),
            })
            .collect();
    }

    rules = rules
        .iter()
        .map(|r| {
            let mut rule = r.clone();
            rule.insert(0, '^');
            rule.push('$');
            rule.replace(" ", "").to_string()
        })
        .collect();

    part1(&rules, &entries);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    // part2(&entries);
    // println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(rules: &Vec<String>, entries: &Vec<&str>) {
    let re = Regex::new(&rules[0]).unwrap();
    println!(
        "Part 1: {:?}.",
        entries.iter().filter(|e| { re.is_match(e) }).count()
    );
}

fn part2(entries: &Vec<String>) {
    println!("Part 2: {:?}.", entries);
}
