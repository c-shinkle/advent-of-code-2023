use day_2::{get_power_of_cube_set, input::INPUT};

fn main() {
    println!("{}", INPUT.lines().map(get_power_of_cube_set).sum::<u32>());
}
