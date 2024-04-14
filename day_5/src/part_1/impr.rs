use super::*;
use std::str::Lines;

fn take_mappings_from_lines(lines: &mut Lines<'_>) -> Vec<Mapping> {
    let mut vec = Vec::new();
    lines.next();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut number_strs = line.split_ascii_whitespace();
        vec.push((
            number_strs.next().unwrap().parse().unwrap(),
            number_strs.next().unwrap().parse().unwrap(),
            number_strs.next().unwrap().parse().unwrap(),
        ));
    }
    vec
}

fn get_mapping(mappings: &[Mapping], value: Index) -> Index {
    for (dest, src, len) in mappings {
        if *src <= value && value < src + len {
            return value - src + dest;
        }
    }
    value
}

pub fn part_1(input: &str) -> Index {
    let mut lines = input.trim().lines();
    let seed_line = lines.next().unwrap();
    // prepare iterator by consuming empty line
    lines.next();

    let seed_to_soil = take_mappings_from_lines(&mut lines);
    let soil_to_fertilizer = take_mappings_from_lines(&mut lines);
    let fertilizer_to_water = take_mappings_from_lines(&mut lines);
    let water_to_light = take_mappings_from_lines(&mut lines);
    let light_to_temp = take_mappings_from_lines(&mut lines);
    let temp_to_humidity = take_mappings_from_lines(&mut lines);
    let humidity_to_location = take_mappings_from_lines(&mut lines);

    let seed_strs = &seed_line[seed_line.find(':').unwrap() + 1..];
    let mut min = Index::MAX;

    for seed_str in seed_strs.split_ascii_whitespace() {
        let seed = seed_str.parse().unwrap();
        let soil = get_mapping(&seed_to_soil, seed);
        let fertilizer = get_mapping(&soil_to_fertilizer, soil);
        let water = get_mapping(&fertilizer_to_water, fertilizer);
        let light = get_mapping(&water_to_light, water);
        let temp = get_mapping(&light_to_temp, light);
        let humidity = get_mapping(&temp_to_humidity, temp);
        let location = get_mapping(&humidity_to_location, humidity);

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
        let actual = part_1(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn impr_puzzle_input() {
        let actual = part_1(INPUT);
        assert_eq!(actual, 107430936);
    }
}
