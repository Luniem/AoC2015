use days::{day3::Day3, day4::Day4};
use util::Solution;

mod days;
mod util;

fn main() {
    // create struct for day
    let curr_day = Day4::default();

    // solve first
    println!("Solution for first part: {}", curr_day.solve_first_part());

    // solve second
    println!("Solution for second part: {}", curr_day.solve_second_part());
}
