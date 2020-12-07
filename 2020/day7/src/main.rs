use std::collections::HashMap;
use regex::Regex;

fn main() {
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    // part2(&entries);
}

fn part1(entries: &Vec<String>) {
  let rules: HashMap<String, Vec<String>> = HashMap::new();
  let contains_bags_re = Regex::new(r"^(\d+)\s([a-z\s]+) bags?\.$").unwrap();

  for l in entries {
    let rule: Vec<&str> = l.split(" bags contain ").collect();

    if rule[1] == "no other bags." {
      return;
    }

    println!("Part 1: {:?}.", rule);
    let contains_bags: Vec<(usize, String)> = rule[1].split(", ").map(|c| {
      let cap = contains_bags_re.captures(c).unwrap();
      (cap[0].parse::<usize>().unwrap(), cap[1].to_string())
    }).collect();
    println!("Part 1: {:?}.", contains_bags);
  }

    println!("Part 1: {:?}.", rules);
}

// fn part2(entries: &Vec<String>) {
//     println!("Part 2: {}.", entries);
// }
