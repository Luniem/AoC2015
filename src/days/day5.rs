use crate::util::{read_input_file_lines, Solution};

pub struct Day5 {
    day: u32,
}

impl Default for Day5 {
    fn default() -> Self {
        Self { day: 5 }
    }
}

impl Solution for Day5 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut nice_string = 0;

        for line in input {
            if is_nice(line) {
                nice_string += 1;
            }
        }

        return nice_string as i64;
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut nice_string = 0;

        for line in input {
            if is_nice2(&line) {
                nice_string += 1;
            }
        }

        return nice_string as i64;
    }
}

fn is_nice(line: String) -> bool {
    return has_vowels(&line) && has_twice(&line) && has_not_forbidden(&line);
}

fn has_vowels(line: &String) -> bool {
    let mut vowel = 0;

    for char in line.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel += 1,
            _ => (),
        };
    }

    return vowel >= 3;
}

fn has_twice(line: &String) -> bool {
    let mut prev = '_';

    for char in line.chars() {
        if char == prev {
            return true;
        }

        prev = char;
    }

    return false;
}

fn has_not_forbidden(line: &String) -> bool {
    let forbidden = vec!["ab", "cd", "pq", "xy"];

    for forbid in forbidden {
        if line.contains(forbid) {
            return false;
        }
    }

    return true;
}

fn is_nice2(line: &String) -> bool {
    return has_non_overlapping_duplicate(line) && has_repeating(line);
}

fn has_non_overlapping_duplicate(line: &String) -> bool {
    let mut index = (0, 1);

    loop {
        let curr_seq = &line[index.0..index.1 + 1];
        let mut sec_index = (0, 1);
        loop {
            let second_seq = &line[sec_index.0..sec_index.1 + 1];

            if index.0 != sec_index.0 && index.1 != sec_index.0 && index.0 != sec_index.1 {
                if curr_seq == second_seq {
                    return true;
                }
            }

            sec_index.0 += 1;
            sec_index.1 += 1;

            if sec_index.1 >= line.len() {
                break;
            }
        }

        index.0 += 1;
        index.1 += 1;

        if index.1 >= line.len() {
            break;
        }
    }

    return false;
}

fn has_repeating(line: &String) -> bool {
    for (i, c) in line.chars().enumerate() {
        if i >= 2 {
            let prev_index = i - 2;
            let prev = &line[prev_index..prev_index + 1];
            if prev == c.to_string() {
                return true;
            }
        }
    }

    return false;
}
