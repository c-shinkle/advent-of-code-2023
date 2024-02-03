mod input;
use crate::input::INPUT;
use day_1::find_number;

fn main() {
    let sum: Option<u32> = INPUT.lines().map(find_number).sum();
    println!("{}", sum.unwrap());
}
