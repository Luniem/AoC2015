use std::{fs::read_to_string, path::Path};

pub trait Solution {
    fn solve_first_part(&self) -> i64;
    fn solve_second_part(&self) -> i64;
}

const SRC_DIR: &str = "src";
const INPUT_DIR: &str = "input";

pub fn read_input_file_as_string(day: u32) -> String {
    let src_path = Path::new(SRC_DIR);
    let input_path = src_path.join(INPUT_DIR);
    let file_path = input_path.join(format!("day{}.txt", day));

    print!("{}", file_path.display());

    let input_file = read_to_string(file_path);
    if input_file.is_ok() {
        return input_file.unwrap();
    }

    panic!("Could not read input-file!");
}

pub fn read_input_file_lines(day: u32) -> Vec<String> {
    let mut result = Vec::new();
    let input_file = read_input_file_as_string(day);

    for line in input_file.lines() {
        result.push(line.to_string())
    }

    return result;
}
