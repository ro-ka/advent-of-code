use std::mem;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let entries = helper::read_file_strings().unwrap();
    let instructions: Vec<(&str, i32)> = entries.iter().map(|e| {
        let (command, value) = e.split_at(1);
        (command, value.parse::<i32>().unwrap())
    }).collect();

    part1(&instructions);
    println!("Time: {}µs", now.elapsed().as_micros());
    now = Instant::now();

    part2(&instructions);
    println!("Time: {}µs", now.elapsed().as_micros());
}

fn part1(instructions: &Vec<(&str, i32)>) {
    let (mut x, mut y, mut direction) = (0, 0, 90);

    for (command, value) in instructions {
      match command {
        &"N" => y += value,
        &"E" => x += value,
        &"S" => y -= value,
        &"W" => x -= value,
        &"L" => direction = (direction - (value % 360) + 360) % 360,
        &"R" => direction = (direction + (value % 360) + 360) % 360,
        &"F" => match direction {
            0 => y += value,
            90 => x += value,
            180 => y -= value,
            270 => x -= value,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
      }
    }

    println!("Part 1: {:?}.", x.abs() + y.abs());
}

fn part2(instructions: &Vec<(&str, i32)>) {
    let (mut x_ship, mut y_ship, mut x_waypoint, mut y_waypoint) = (0, 0, 10, 1);

    for (command, value) in instructions {
      match command {
        &"N" => y_waypoint += value,
        &"E" => x_waypoint += value,
        &"S" => y_waypoint -= value,
        &"W" => x_waypoint -= value,
        &"R" => {
            for _ in 0..(value/90) {
                x_waypoint *= -1;
                mem::swap(&mut x_waypoint, &mut y_waypoint);
            }
        },
        &"L" => {
            for _ in 0..(value/90) {
                y_waypoint *= -1;
                mem::swap(&mut x_waypoint, &mut y_waypoint);
            }
        },
        &"F" => {
          x_ship += value * x_waypoint;
          y_ship += value * y_waypoint;
        },
        _ => unimplemented!(),
      }
    }

    println!("Part 2: {:?}.", x_ship.abs() + y_ship.abs());
}
