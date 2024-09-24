use days::day2::Day2;
use util::Solution;

mod days;
mod util;

fn main() {
    // create struct for day
    let curr_day = Day2::default();

    // solve first
    println!("Solution for first part: {}", curr_day.solve_first_part());

    // solve second
    println!("Solution for second part: {}", curr_day.solve_second_part());
}
