use std::{collections::HashMap, str::Lines};

fn map_line(lines: &mut Lines<'_>) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    lines.next();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut number_strs = line.split_ascii_whitespace();
        let dest: u64 = number_strs.next().unwrap().parse().unwrap();
        let source: u64 = number_strs.next().unwrap().parse().unwrap();
        let len: u64 = number_strs.next().unwrap().parse().unwrap();
        for offset in 0..len {
            map.insert(source + offset, dest + offset);
        }
    }
    map
}

pub fn part_1_impr(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let seed_line = lines.next().unwrap();
    // prep iterator by consuming empty line
    lines.next();

    let seed_to_soil = map_line(&mut lines);
    let soil_to_fertilizer = map_line(&mut lines);
    let fertilizer_to_water = map_line(&mut lines);
    let water_to_light = map_line(&mut lines);
    let light_to_temp = map_line(&mut lines);
    let temp_to_humidity = map_line(&mut lines);
    let humidity_to_location = map_line(&mut lines);

    let mut min: Option<u64> = None;
    let seeds = seed_line[seed_line.find(':').unwrap() + 1..]
        .split_ascii_whitespace()
        .map(|seed_str| seed_str.parse().unwrap());
    for seed in seeds {
        let soil = seed_to_soil.get(&seed).unwrap_or(&seed);
        let fertilizer = soil_to_fertilizer.get(soil).unwrap_or(soil);
        let water = fertilizer_to_water.get(fertilizer).unwrap_or(fertilizer);
        let light = water_to_light.get(water).unwrap_or(water);
        let temp = light_to_temp.get(light).unwrap_or(light);
        let humidity = temp_to_humidity.get(temp).unwrap_or(temp);
        let location = humidity_to_location.get(humidity).unwrap_or(humidity);

        if min.is_none() || min.unwrap().gt(location) {
            min = Some(*location);
        }
    }

    min.unwrap()
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
56 93 4";

        let actual = part_1_impr(input);
        assert_eq!(actual, 35);
    }

    #[test]
    fn impr_puzzle_input() {
        let actual = part_1_impr(INPUT);
        assert_eq!(actual, 0);
    }
}
