use std::str::Lines;

pub type Index = u64;
type Mapping = (Index, Index, Index);
struct MappingStruct {
    dest: Index,
    src: Index,
    len: Index,
}

impl FromIterator<Index> for MappingStruct {
    fn from_iter<T: IntoIterator<Item = Index>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();

        MappingStruct {
            dest: into_iter.next().unwrap(),
            src: into_iter.next().unwrap(),
            len: into_iter.next().unwrap(),
        }
    }
}

impl From<MappingStruct> for Mapping {
    fn from(MappingStruct { dest, src, len }: MappingStruct) -> Self {
        (dest, src, len)
    }
}

fn take_mapping_from_lines(lines: &mut Lines<'_>) -> Vec<Mapping> {
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

    let seed_to_soil = take_mapping_from_lines(&mut lines);
    let soil_to_fertilizer = take_mapping_from_lines(&mut lines);
    let fertilizer_to_water = take_mapping_from_lines(&mut lines);
    let water_to_light = take_mapping_from_lines(&mut lines);
    let light_to_temp = take_mapping_from_lines(&mut lines);
    let temp_to_humidity = take_mapping_from_lines(&mut lines);
    let humidity_to_location = take_mapping_from_lines(&mut lines);

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

fn get_mapping(lines: &[&str]) -> Vec<Mapping> {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number_str| number_str.parse::<Index>().unwrap())
                .collect::<MappingStruct>()
                .into()
        })
        .collect()
}

fn map_to_func(mappings: &[Mapping], value: Index) -> Index {
    mappings
        .iter()
        .find(|&&(_, src, len)| (src..src + len).contains(&value))
        .map(|&(dest, src, _)| value - src + dest)
        .unwrap_or(value)
}

pub fn part_1_func(input: &str) -> Index {
    let lines: Vec<&str> = input.trim().lines().collect();
    let list_of_mappings: Vec<Vec<Mapping>> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("map:"))
        .map(|(offset, _)| get_mapping(&lines[(offset + 1)..]))
        .collect();

    let seed_line = lines[0];
    seed_line[(seed_line.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|seed_str| {
            let seed = seed_str.parse().unwrap();

            let soil = map_to_func(&list_of_mappings[0], seed);
            let fertilizer = map_to_func(&list_of_mappings[1], soil);
            let water = map_to_func(&list_of_mappings[2], fertilizer);
            let light = map_to_func(&list_of_mappings[3], water);
            let temp = map_to_func(&list_of_mappings[4], light);
            let humidity = map_to_func(&list_of_mappings[5], temp);
            map_to_func(&list_of_mappings[6], humidity)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::INPUT;

    const EXAMPLE: &str = "
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

    #[test]
    fn impr_sample() {
        let actual = part_1_impr(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn func_sample() {
        let actual = part_1_func(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn impr_puzzle_input() {
        let actual = part_1_impr(INPUT);
        assert_eq!(actual, 107430936);
    }

    #[test]
    fn func_puzzle_input() {
        let actual = part_1_func(INPUT);
        assert_eq!(actual, 107430936);
    }
}
