use std::collections::HashMap;
use std::time::Instant;

static RAW_NUMBERS: &str = "0,20,7,16,1,18,15";

fn main() {
    let mut now = Instant::now();

    println!("Part 1: {:?}.", find_number(2020));
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    println!("Part 2: {:?}.", find_number(30000000));
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn find_number(iterations: u32) -> u32 {
    let mut occurances: HashMap<u32, u32> = HashMap::new();
    let mut latest = 0;

    RAW_NUMBERS
        .split(",")
        .map(|n| n.parse().expect("Could not parse number!"))
        .enumerate()
        .for_each(|(i, n)| {
            latest = n;
            occurances.insert(n, i as u32);
        });

    occurances.remove(&latest);

    for i in occurances.len() + 1..iterations as usize {
        match occurances.get(&latest) {
            Some(index) => {
                let new = i as u32 - 1 - index;
                occurances.insert(latest, i as u32 - 1);
                latest = new;
            }
            None => {
                occurances.insert(latest, i as u32 - 1);
                latest = 0;
            }
        }
    }

    latest
}
