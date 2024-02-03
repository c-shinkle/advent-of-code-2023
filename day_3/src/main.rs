use day_3::get_all_part_numbers;
use day_3::input::INPUT;
use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let sum: u32 = get_all_part_numbers(INPUT.trim()).into_iter().sum();
    println!("{sum}");
}
