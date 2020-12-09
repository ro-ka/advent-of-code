use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let numbers = helper::read_file_i32s().unwrap();

    part1(&numbers);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();
    part2(&numbers);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(numbers: &Vec<i32>) {
    for i in numbers {
        for k in numbers {
            match i + k == 2020 {
                true => {
                    println!("Part1: {} * {} = {}", i, k, i * k);
                    return;
                }
                false => (),
            }
        }
    }
}

fn part2(numbers: &Vec<i32>) {
    for i in numbers {
        for k in numbers {
            for l in numbers {
                match i + k + l == 2020 {
                    true => {
                        println!("Part2: {} * {} * {} = {}", i, k, l, i * k * l);
                        return;
                    }
                    false => (),
                }
            }
        }
    }
}
