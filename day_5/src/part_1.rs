use itertools::Itertools;
use std::str::Lines;

fn map_line(lines: &mut Lines<'_>) -> Vec<(u64, u64, u64)> {
    let mut vec = Vec::new();
    lines.next();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut number_strs = line.split_ascii_whitespace();
        let dest: u64 = number_strs.next().unwrap().parse().unwrap();
        let src: u64 = number_strs.next().unwrap().parse().unwrap();
        let len: u64 = number_strs.next().unwrap().parse().unwrap();
        vec.push((dest, src, len));
    }
    vec
}

fn get_mapping_impr(mappings: &[(u64, u64, u64)], value: u64) -> u64 {
    for (dest, src, len) in mappings.iter().copied() {
        if src <= value && value < src + len {
            return value - src + dest;
        }
    }
    value
}

pub fn part_1_impr(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let seed_line = lines.next().unwrap();
    // prepare iterator by consuming empty line
    lines.next();

    let seed_to_soil = map_line(&mut lines);
    let soil_to_fertilizer = map_line(&mut lines);
    let fertilizer_to_water = map_line(&mut lines);
    let water_to_light = map_line(&mut lines);
    let light_to_temp = map_line(&mut lines);
    let temp_to_humidity = map_line(&mut lines);
    let humidity_to_location = map_line(&mut lines);

    let mut min = u64::MAX;
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

pub fn get_mapping_with_next_index(lines: &[&str], index: usize) -> (Vec<(u64, u64, u64)>, usize) {
    let mappings: Vec<(u64, u64, u64)> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number_str| number_str.parse().unwrap())
                .collect_tuple::<(u64, u64, u64)>()
                .unwrap()
        })
        .collect();
    let len = mappings.len() + 2;
    (mappings, index + len)
}

fn get_mapping_func(mappings: &[(u64, u64, u64)], value: u64) -> u64 {
    mappings
        .iter()
        .find(|&&(_, src, len)| (src..src + len).contains(&value))
        .map(|&(dest, src, _)| value - src + dest)
        .unwrap_or(value)
}

pub fn part_1_func(input: &str) -> u64 {
    let input = input.trim();
    let lines: Vec<&str> = input.lines().collect();
    let mut offset = 3;
    let list_of_mappings: Vec<Vec<(u64, u64, u64)>> = (0..7)
        .map(|_| {
            let (mapping, next_offset) = get_mapping_with_next_index(&lines[offset..], offset);
            offset = next_offset;
            mapping
        })
        .collect();

    let seed_line = lines[0];
    seed_line[(seed_line.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|seed_str| {
            let seed = seed_str.parse().unwrap();

            let soil = get_mapping_func(&list_of_mappings[0], seed);
            let fertilizer = get_mapping_func(&list_of_mappings[1], soil);
            let water = get_mapping_func(&list_of_mappings[2], fertilizer);
            let light = get_mapping_func(&list_of_mappings[3], water);
            let temp = get_mapping_func(&list_of_mappings[4], light);
            let humidity = get_mapping_func(&list_of_mappings[5], temp);
            get_mapping_func(&list_of_mappings[6], humidity)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::INPUT;

    #[test]
    fn impr_sample() {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let actual = part_1_impr(input);
        assert_eq!(actual, 35);
    }

    #[test]
    fn func_sample() {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let actual = part_1_func(input);
        assert_eq!(actual, 35);
    }

    #[test]
    fn impr_puzzle_input() {
        let actual = part_1_func(INPUT);
        assert_eq!(actual, 107430936);
    }
}
