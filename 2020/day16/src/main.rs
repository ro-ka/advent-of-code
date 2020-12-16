use std::collections::HashMap;
use std::fs;
use std::time::Instant;

static TICKET: &str = "151,139,53,71,191,107,61,109,157,131,67,73,59,79,113,167,137,163,149,127";

fn main() {
    let mut now = Instant::now();

    let mut rules: HashMap<&str, Vec<std::ops::RangeInclusive<u32>>> = HashMap::new();
    let raw_rules = fs::read_to_string("rules.txt").unwrap();
    raw_rules.lines().for_each(|l| {
        let rule: Vec<&str> = l.split(": ").collect();
        rules.insert(
            rule[0],
            rule[1]
                .split(" or ")
                .map(|r| {
                    let values: Vec<&str> = r.split("-").collect();
                    values[0].parse().unwrap()..=values[1].parse().unwrap()
                })
                .collect(),
        );
    });

    let ticket: Vec<u32> = TICKET.split(",").map(|v| v.parse().unwrap()).collect();

    let raw_nearby = fs::read_to_string("nearby.txt").unwrap();
    let nearby: Vec<Vec<u32>> = raw_nearby
        .lines()
        .map(|l| l.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    part1(&rules, &nearby);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&rules, &nearby, &ticket);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(rules: &HashMap<&str, Vec<std::ops::RangeInclusive<u32>>>, nearby: &Vec<Vec<u32>>) {
    println!(
        "Part 1: {:?}.",
        nearby
            .iter()
            .map(|n| is_invalid(n, rules))
            .filter(|(invalid, _)| *invalid)
            .map(|(_, number)| number)
            .sum::<u32>()
    );
}

fn part2(
    rules: &HashMap<&str, Vec<std::ops::RangeInclusive<u32>>>,
    nearby: &Vec<Vec<u32>>,
    ticket: &Vec<u32>,
) {
    let valid_nearby: Vec<&Vec<u32>> = nearby
        .iter()
        .filter(|n| {
            let (invalid, _) = is_invalid(n, rules);
            !invalid
        })
        .collect();

    let mut rules_indices: HashMap<&str, Vec<usize>> = HashMap::new();
    for i in 0..ticket.len() {
        for (rule_name, rule) in rules {
            if valid_nearby
                .iter()
                .all(|n| rule.iter().any(|r| r.contains(&n[i])))
            {
                rules_indices.entry(rule_name).or_insert(vec![]).push(i);
            }
        }
    }

    let mut single_indices: Vec<usize> = rules_indices
        .values()
        .filter(|r| r.len() == 1)
        .flatten()
        .map(|r| *r)
        .collect::<Vec<usize>>();

    while single_indices.len() < ticket.len() {
        for (_, indices) in rules_indices.iter_mut() {
            if indices.len() > 1 {
                *indices = indices
                    .iter()
                    .filter(|i| !single_indices.contains(&i))
                    .map(|i| *i)
                    .collect::<Vec<usize>>();

                if indices.len() == 1 {
                    single_indices.push(indices[0]);
                }
            }
        }
    }

    println!(
        "Part 2: {:?}.",
        rules_indices
            .iter()
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, i)| i[0])
            .map(|i| ticket[i] as u64)
            .inspect(|i| println!("{:?}", i))
            .product::<u64>()
    );
}

fn is_invalid(
    nearby: &Vec<u32>,
    rules: &HashMap<&str, Vec<std::ops::RangeInclusive<u32>>>,
) -> (bool, u32) {
    match nearby.iter().find(|i| {
        rules
            .values()
            .all(|rule| !rule.iter().any(|r| r.contains(i)))
    }) {
        Some(v) => (true, *v),
        None => (false, 0),
    }
}
