use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let raw_data = helper::read_file_to_string().unwrap();
    let data: Vec<&str> = raw_data.split("\n\n").collect();

    let mut rules: HashMap<String, String> = HashMap::new();
    for l in data[0].lines() {
        let parts: Vec<&str> = l.split(": ").collect();
        rules.insert(parts[0].to_string(), parts[1].replace("\"", ""));
    }

    let index_re = Regex::new(r"(\d+)").unwrap();

    let mut changed = true;
    while changed {
        changed = false;
        for (i, r) in rules {
            match index_re.captures(&r) {
              Some(cap) => {
                  let index = cap[1].to_string();
                  rules.insert(i, r.replace(&index, rules.get(&index).unwrap()));
                  changed = true;
              },
              None => (),
            }
        }
    }

    println!("{:?}", rules);

    // part1(&entries);
    // println!("Time: {}µs", now.elapsed().as_micros());
    // now = Instant::now();

    // part2(&entries);
    // println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(entries: &Vec<String>) {
    println!("Part 1: {:?}.", entries);
}

fn part2(entries: &Vec<String>) {
    println!("Part 2: {:?}.", entries);
}
