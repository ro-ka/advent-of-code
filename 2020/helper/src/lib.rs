use std::fs;

pub fn read_file_i32s() -> std::io::Result<Vec<i32>> {
    let data = fs::read_to_string("input.txt")?
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    return Ok(data);
}

pub fn read_file_to_string() -> std::io::Result<String> {
    fs::read_to_string("input.txt")
}

pub fn read_file_strings() -> std::io::Result<Vec<String>> {
    let data = fs::read_to_string("input.txt")?
        .lines()
        .map(|v| v.to_string())
        .collect();
    return Ok(data);
}
