use std::unimplemented;

fn main() {
    let entries = helper::read_file_strings().unwrap();

    let instructions: Vec<(&str, i32)> = entries
        .iter()
        .map(|i| {
            let instruction: Vec<&str> = i.split_whitespace().collect();
            (instruction[0], instruction[1].parse::<i32>().unwrap())
        })
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<(&str, i32)>) {
    let (acc, _) = run(instructions, 0);
    println!("Part 1: {:?}.", acc);
}

fn part2(instructions: &Vec<(&str, i32)>) {
    for i in 0..instructions.len() {
        let (acc, did_finish) = run(instructions, i);
        if did_finish {
            println!("Part 2: {:?}.", acc);
            return;
        }
    }
}

fn run(instructions: &Vec<(&str, i32)>, flip_index: usize) -> (i32, bool) {
    let mut called_instructions: Vec<usize> = Vec::new();
    let mut next_instruction_index: usize = 0;
    let mut acc: i32 = 0;

    while next_instruction_index < instructions.len()
        && !called_instructions.contains(&next_instruction_index)
    {
        let (operation, value) = instructions[next_instruction_index];
        called_instructions.push(next_instruction_index);

        let operation_to_execute = match flip_index != 0 && flip_index == next_instruction_index {
            true => match operation {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => operation,
            },
            false => operation,
        };

        match operation_to_execute {
            "nop" => next_instruction_index += 1,
            "acc" => {
                next_instruction_index += 1;
                acc += value;
            }
            "jmp" => next_instruction_index = (next_instruction_index as i32 + value) as usize,
            _ => unimplemented!(),
        }
    }

    (acc, next_instruction_index >= instructions.len())
}
