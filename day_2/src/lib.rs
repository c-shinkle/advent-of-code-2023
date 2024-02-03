use std::cmp::max;

pub mod input;

pub fn sum_of_winning_games(line: &str) -> u32 {
    let mut split_on_colon = line.split(':');
    let id = &split_on_colon.next().unwrap()[5..];
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for round in split_on_colon.next().unwrap().split(';') {
        for number_with_color in round.split(',') {
            let mut number_with_color_iter = number_with_color.split_ascii_whitespace();
            let number: u32 = number_with_color_iter.next().unwrap().parse().unwrap();
            let color = number_with_color_iter.next().unwrap();
            let x = match color {
                "red" => &mut red,
                "green" => &mut green,
                "blue" => &mut blue,
                _ => panic!("Unaccounted for color: {color}"),
            };
            *x = max(*x, number);
        }
    }

    if red <= 12 && green <= 13 && blue <= 14 {
        return id.parse().unwrap();
    }
    0
}

pub fn get_power_of_cube_set(line: &str) -> u32 {
    let mut split_on_colon = line.split(':');
    split_on_colon.next();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for round in split_on_colon.next().unwrap().split(';') {
        for number_with_color in round.split(',') {
            let mut number_with_color_iter = number_with_color.split_ascii_whitespace();
            let number: u32 = number_with_color_iter.next().unwrap().parse().unwrap();
            let color = number_with_color_iter.next().unwrap();
            let x = match color {
                "red" => &mut red,
                "green" => &mut green,
                "blue" => &mut blue,
                _ => panic!("Unaccounted for color: {color}"),
            };
            *x = max(*x, number);
        }
    }

    red * green * blue
}
