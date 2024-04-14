use super::*;

struct MappingStruct {
    dest: Index,
    src: Index,
    len: Index,
}

impl<'a> FromIterator<&'a str> for MappingStruct {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();

        MappingStruct {
            dest: into_iter.next().unwrap().parse().unwrap(),
            src: into_iter.next().unwrap().parse().unwrap(),
            len: into_iter.next().unwrap().parse().unwrap(),
        }
    }
}

fn get_mapping(mappings: &[MappingStruct], value: Index) -> Index {
    mappings
        .iter()
        .find(|&&MappingStruct { src, len, .. }| (src..src + len).contains(&value))
        .map(|MappingStruct { dest, src, .. }| value - src + dest)
        .unwrap_or(value)
}

pub fn part_1(input: &str) -> Index {
    let lines: Vec<&str> = input.trim().lines().collect();
    let list_of_mappings: Vec<Vec<MappingStruct>> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("map:"))
        .map(|(offset, _)| {
            lines[(offset + 1)..]
                .iter()
                .take_while(|line| !line.is_empty())
                .map(|line| line.split_ascii_whitespace().collect())
                .collect()
        })
        .collect();

    let seed_to_soil = &list_of_mappings[0];
    let soil_to_fertilizer = &list_of_mappings[1];
    let fertilizer_to_water = &list_of_mappings[2];
    let water_to_light = &list_of_mappings[3];
    let light_to_temp = &list_of_mappings[4];
    let temp_to_humidity = &list_of_mappings[5];
    let humidity_to_location = &list_of_mappings[6];

    let seed_line = lines[0];
    seed_line[(seed_line.find(':').unwrap() + 1)..]
        .split_ascii_whitespace()
        .map(|seed_str| {
            let seed = seed_str.parse().unwrap();

            let soil = get_mapping(seed_to_soil, seed);
            let fertilizer = get_mapping(soil_to_fertilizer, soil);
            let water = get_mapping(fertilizer_to_water, fertilizer);
            let light = get_mapping(water_to_light, water);
            let temp = get_mapping(light_to_temp, light);
            let humidity = get_mapping(temp_to_humidity, temp);
            get_mapping(humidity_to_location, humidity)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::INPUT;

    #[test]
    fn func_sample() {
        let actual = part_1(EXAMPLE);
        assert_eq!(actual, 35);
    }

    #[test]
    fn func_puzzle_input() {
        let actual = part_1(INPUT);
        assert_eq!(actual, 107430936);
    }
}
