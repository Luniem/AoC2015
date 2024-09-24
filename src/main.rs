use days::day1::Day1;
use util::Solution;

mod days;
mod util;

fn main() {
    // create struct for day
    let curr_day = Day1::default();

    // solve first
    println!("Solution for first part: {}", curr_day.solve_first_part());

    // solve second
    println!("Solution for second part: {}", curr_day.solve_second_part());
}
