use crate::util::{read_input_file_as_string, Solution};

pub struct Day4 {
    day: u32,
}

impl Default for Day4 {
    fn default() -> Self {
        Self { day: 4 }
    }
}

impl Solution for Day4 {
    fn solve_first_part(&self) -> i64 {
        let sec_key = read_input_file_as_string(self.day);
        let mut i = 0;

        loop {
            let key = format!("{}{}", sec_key, i);
            let hash = md5::compute(key);
            let hex_hash = format!("{:x}", hash);
            let first_five = &hex_hash[..5];

            if first_five == "00000" {
                return i as i64;
            }

            i += 1;
        }
    }

    fn solve_second_part(&self) -> i64 {
        let sec_key = read_input_file_as_string(self.day);
        let mut i = 0;

        loop {
            let key = format!("{}{}", sec_key, i);
            let hash = md5::compute(key);
            let hex_hash = format!("{:x}", hash);
            let first_six = &hex_hash[..6];

            if first_six == "000000" {
                return i as i64;
            }

            i += 1;
        }
    }
}
