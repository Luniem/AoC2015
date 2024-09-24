use std::{char, collections::HashSet};

use crate::util::{read_input_file_as_string, Solution};

pub struct Day3 {
    day: u32,
}

impl Default for Day3 {
    fn default() -> Self {
        Self { day: 3 }
    }
}

impl Solution for Day3 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_as_string(self.day);
        let mut house_set: HashSet<(i32, i32)> = HashSet::new();

        let mut x = 0;
        let mut y = 0;

        house_set.insert((x, y));

        for char in input.chars() {
            match char {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => panic!("Not expected char!"),
            };

            house_set.insert((x, y));
        }

        return house_set.len() as i64;
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_as_string(self.day);

        let mut santa = (0, 0);
        let mut robot = (0, 0);

        let mut house_set: HashSet<(i32, i32)> = HashSet::new();
        house_set.insert(santa);

        for (i, char) in input.chars().enumerate() {
            if i % 2 == 0 {
                santa = next_instruction(santa, char);
            } else {
                robot = next_instruction(robot, char);
            }

            house_set.insert(santa);
            house_set.insert(robot);
        }

        return house_set.len() as i64;
    }
}

fn next_instruction(mut curr_chords: (i32, i32), char: char) -> (i32, i32) {
    match char {
        '^' => curr_chords.1 += 1,
        'v' => curr_chords.1 -= 1,
        '>' => curr_chords.0 += 1,
        '<' => curr_chords.0 -= 1,
        _ => panic!("Not expected char!"),
    };

    curr_chords
}
