use crate::util::{read_input_file_as_string, Solution};

pub struct Day1 {
    day: u32,
}

impl Default for Day1 {
    fn default() -> Self {
        Self { day: 1 }
    }
}

impl Solution for Day1 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_as_string(self.day);

        let mut floor = 0;

        for char in input.chars() {
            if char == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }
        }

        floor
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_as_string(self.day);

        let mut floor = 0;

        for (index, char) in input.chars().enumerate() {
            if char == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }

            if floor == -1 {
                return (index + 1) as i64;
            }
        }

        panic!("Should have finished!");
    }
}
