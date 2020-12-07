use regex::Regex;
use std::collections::HashMap;

fn main() {
    let entries = helper::read_file_strings().unwrap();

    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let contains_bags_re = Regex::new(r"^(\d+) ([a-z ]+)( bags?.?)").unwrap();

    for l in entries {
        let rule: Vec<&str> = l.split(" bags contain ").collect();

        if rule[1] == "no other bags." {
            continue;
        }

        let contains_bags: Vec<(usize, String)> = rule[1]
            .split(", ")
            .map(|c| {
                let cap = contains_bags_re.captures(c).unwrap();
                (cap[1].parse::<usize>().unwrap(), cap[2].to_string())
            })
            .collect();

        rules.insert(rule[0].to_string(), contains_bags);
    }

    part1(&rules);
    part2(&rules);
}

fn part1(rules: &HashMap<String, Vec<(usize, String)>>) {
    let mut possible_bags_containing_shiny_gold = 0;
    for (bag, _) in rules {
        if bag != "shiny gold" && can_contain_shiny_gold_bag(bag, rules) {
            possible_bags_containing_shiny_gold += 1;
        }
    }

    println!("Part 1: {:?}.", possible_bags_containing_shiny_gold);
}

fn part2(rules: &HashMap<String, Vec<(usize, String)>>) {
    println!("Part 2: {:?}.", get_containing_bags("shiny gold", rules));
}

fn can_contain_shiny_gold_bag(bag: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> bool {
    if bag == "shiny gold" {
        return true;
    }

    match rules.get(bag) {
        Some(containing_bags) => {
            return containing_bags
                .iter()
                .any(|(_, b)| can_contain_shiny_gold_bag(b, rules))
        }
        None => return false,
    }
}

fn get_containing_bags(bag: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
    match rules.get(bag) {
        Some(containing_bags) => {
            return containing_bags
                .iter()
                .map(|(count, b)| match get_containing_bags(b, rules) {
                    0 => return count * 1,
                    containing_bags_count => count * containing_bags_count + count,
                })
                .fold(0, |acc, x| acc + x)
        }
        None => return 0,
    }
}
