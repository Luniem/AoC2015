use crate::util::{read_input_file_as_string, read_input_file_lines, Solution};

pub struct Day2 {
    day: u32,
}

impl Default for Day2 {
    fn default() -> Self {
        Self { day: 2 }
    }
}

impl Solution for Day2 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut wrapping_paper = 0;

        for line in input {
            let parse_result = parse_box_dimension(line);
            if parse_result.is_none() {
                panic!("Could not parse dimensions!");
            }

            let (l, w, h) = parse_result.unwrap();
            let first_side = l * w;
            let second_side = w * h;
            let third_side = h * l;

            let surface_area_box = (2 * first_side) + (2 * second_side) + (2 * third_side);

            let mut side_vec = vec![first_side, second_side, third_side];
            side_vec.sort();
            let smallest_side = side_vec.first().unwrap();

            wrapping_paper += surface_area_box + smallest_side;
        }

        return wrapping_paper as i64;
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut ribbon = 0;

        for line in input {
            let parse_result = parse_box_dimension(line);
            if parse_result.is_none() {
                panic!("Could not parse dimensions!");
            }

            let (l, w, h) = parse_result.unwrap();

            let mut side_vec = vec![l, w, h];
            side_vec.sort();
            let smallest_side = side_vec.first().unwrap();
            let second_smallest = side_vec[1];

            ribbon += 2 * (smallest_side + second_smallest) + (l * w * h);
        }

        return ribbon as i64;
    }
}

fn parse_box_dimension(line: String) -> Option<(i32, i32, i32)> {
    let (l, rest) = line.split_once("x")?;
    let (w, h) = rest.split_once("x")?;

    let l = l.parse::<i32>().ok()?;
    let w = w.parse::<i32>().ok()?;
    let h = h.parse::<i32>().ok()?;

    Some((l, w, h))
}
