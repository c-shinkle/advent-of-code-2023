use super::*;
use std::str::Lines;

fn take_mappings_from_lines_impr(lines: &mut Lines<'_>) -> Vec<Mapping> {
    lines.next();
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut number_strs = line.split_ascii_whitespace();
            (
                number_strs.next().unwrap().parse().unwrap(),
                number_strs.next().unwrap().parse().unwrap(),
                number_strs.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn get_mapping_impr(mappings: &[Mapping], value: Index) -> Index {
    for (dest, src, len) in mappings {
        if *src <= value && value < src + len {
            return value - src + dest;
        }
    }
    value
}

pub fn part_1_impr(input: &str) -> Index {
    let mut lines = input.trim().lines();
    let seed_line = lines.next().unwrap();
    // prepare iterator by consuming empty line
    lines.next();

    let seed_to_soil = take_mappings_from_lines_impr(&mut lines);
    let soil_to_fertilizer = take_mappings_from_lines_impr(&mut lines);
    let fertilizer_to_water = take_mappings_from_lines_impr(&mut lines);
    let water_to_light = take_mappings_from_lines_impr(&mut lines);
    let light_to_temp = take_mappings_from_lines_impr(&mut lines);
    let temp_to_humidity = take_mappings_from_lines_impr(&mut lines);
    let humidity_to_location = take_mappings_from_lines_impr(&mut lines);

    let mut min = Index::MAX;
    for seed in seed_line[seed_line.find(':').unwrap() + 1..]
        .split_ascii_whitespace()
        .map(|seed_str| seed_str.parse().unwrap())
    {
        let soil = get_mapping_impr(&seed_to_soil, seed);
        let fertilizer = get_mapping_impr(&soil_to_fertilizer, soil);
        let water = get_mapping_impr(&fertilizer_to_water, fertilizer);
        let light = get_mapping_impr(&water_to_light, water);
        let temp = get_mapping_impr(&light_to_temp, light);
        let humidity = get_mapping_impr(&temp_to_humidity, temp);
        let location = get_mapping_impr(&humidity_to_location, humidity);

        min = min.min(location);
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::INPUT;

    #[test]
    fn impr_sample() {
        let actual = part_1_impr(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn impr_puzzle_input() {
        let actual = part_1_impr(INPUT);
        assert_eq!(actual, 107430936);
    }
}
