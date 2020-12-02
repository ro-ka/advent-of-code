use std::fs;

fn main() {
    let numbers = read_file().unwrap();

    part1(&numbers);
    part2(&numbers);
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

fn read_file() -> std::io::Result<Vec<i32>> {
    let data = fs::read_to_string("input.txt")?
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    return Ok(data);
}
