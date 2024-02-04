// use day_3::get_all_part_numbers;
use day_3::input::INPUT;
use itertools::Itertools;

fn main() {
    // let sum: u32 = get_all_part_numbers(INPUT.trim()).into_iter().sum();
    // println!("{sum}");

    println!("{}", INPUT.trim().chars().unique().join(", "));
}
