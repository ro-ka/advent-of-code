use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    // let time = 939;
    // let bus_lines = "7,13,x,x,59,x,31,19";
    let time = 1015292;
    let bus_lines = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,743,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,643,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

    part1(time, bus_lines);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(bus_lines);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(time: u32, raw_bus_lines: &str) {
    let bus_lines: Vec<u32> = raw_bus_lines
        .split(",")
        .filter(|c| c != &"x")
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    let next_bus_at = bus_lines
        .iter()
        .map(|i| {
            let mut departure_time = 0;
            while departure_time < time {
                departure_time += i
            }
            departure_time
        })
        .min()
        .expect("No departure time found!");
    let bus_id = bus_lines
        .iter()
        .find(|i| next_bus_at % **i == 0)
        .expect("No bus id found!");
    let time_to_wait = next_bus_at - time;

    println!("Part 1: {:?}.", bus_id * time_to_wait);
}

fn part2(raw_bus_lines: &str) {
    let rules: Vec<(usize, u32)> = raw_bus_lines
        .split(",")
        .enumerate()
        .map(|(i, c)| match c {
            "x" => (i, 0),
            _ => (i, c.parse::<u32>().unwrap()),
        })
        .filter(|(_, c)| c > &0u32)
        .collect();

    let max_bus_id = rules.iter().map(|(_, c)| c).max().expect("No max found!");

    let start = max_bus_id
        - rules
            .iter()
            .position(|(_, c)| c == max_bus_id)
            .expect("Position of bus id not found!") as u32;

    println!("{:?}", rules);

    let time = (start..)
        .into_iter()
        .step_by(*max_bus_id as usize)
        // .find(|i| (i + 4) % 59 == 0 && (i + 6) % 31 == 0)
        .find(|i| rules.iter().all(|(k, c)| (i + *k as u32) % c == 0))
        .expect("Could not find a time.  :(");

    println!("Part 2: {:?}.", time);
}
