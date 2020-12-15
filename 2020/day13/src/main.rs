use std::time::Instant;

static TIME: u32 = 939;
static BUS_LINES: &str = "7,13,x,x,59,x,31,19";

// static TIME: u32 = 1015292;
// static BUS_LINES: &str = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,743,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,643,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

fn main() {
    let mut now = Instant::now();

    part1();
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2();
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1() {
    let (bus_id, time_to_wait) = BUS_LINES
        .split(",")
        .filter(|c| c != &"x")
        .map(|c| c.parse::<u32>().unwrap())
        .map(|i| (i, i - (TIME % i)))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .expect("No departure time found!");

    println!("Part 1: {:?}.", bus_id * time_to_wait);
}

fn part2() {
    let mut rules: Vec<(usize, u64)> = BUS_LINES
        .split(",")
        .enumerate()
        .map(|(index, n)| match n {
            "x" => (index, 0),
            _ => (index, n.parse::<u64>().unwrap()),
        })
        .filter(|(_, n)| n > &0u64)
        .collect();

    rules.sort_by(|(_, a), (_, b)| a.cmp(b));
    rules.reverse();

    let (max_index, max_id) = rules[0];
    // let (second_max_index, second_max_id) = rules[1];

    let start = max_id - max_index as u64;

    // let lcm: Vec<u64> = (max_id as u64..100000)
    //     .into_iter()
    //     .step_by(max_id as usize)
    //     .filter(|i| second_max_id - (i % second_max_id) == (second_max_index - max_index) as u64)
    //     .collect();
    // // .expect("lcm not found!");

    // println!("START {:?}", start);
    // println!("LCM {:?}", lcm);

    let time = (start..)
        .into_iter()
        .step_by(max_id as usize)
        .find(|i| {
            rules
                .iter()
                .all(|(index, id)| (i + *index as u64) % id == 0)
        })
        .expect("Could not find a time.  :(");

    println!("Part 2: {:?}.", time);
}
