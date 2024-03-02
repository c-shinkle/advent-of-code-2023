use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    start: usize,
    end: usize,
    row: usize,
}

type LocationTuple = (u16, u16, u16);

pub fn first_impl_gear_ratios(mut input: &str) -> u32 {
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    input = input.trim();
    let matrix: Vec<&str> = input.lines().collect();
    let all_locations: Vec<Location> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            digit_regex
                .find_iter(line)
                .map(move |capture| (row, capture))
        })
        .map(|(row, capture)| Location {
            start: capture.start(),
            end: capture.end(),
            row,
        })
        .collect();
    let asterisk_regex = Regex::new(r"(\*)").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            asterisk_regex
                .find_iter(line)
                .map(move |capture| (row, capture.start()))
        })
        .map(|(asterisk_row, asterisk_col)| {
            let mut set: HashSet<Location> = HashSet::new();
            [
                (Some(asterisk_row), asterisk_col.checked_add(1)),
                (asterisk_row.checked_sub(1), asterisk_col.checked_add(1)),
                (asterisk_row.checked_sub(1), Some(asterisk_col)),
                (asterisk_row.checked_sub(1), asterisk_col.checked_sub(1)),
                (Some(asterisk_row), asterisk_col.checked_sub(1)),
                (asterisk_row.checked_add(1), asterisk_col.checked_sub(1)),
                (asterisk_row.checked_add(1), Some(asterisk_col)),
                (asterisk_row.checked_add(1), asterisk_col.checked_add(1)),
            ]
            .into_iter()
            .filter_map(|(row, col)| Some((row?, col?)))
            .for_each(|(row, col)| {
                for location in all_locations.iter() {
                    let Location {
                        start,
                        end,
                        row: digit_row,
                    } = location;
                    let range = *start..*end;
                    if row == *digit_row && range.contains(&col) {
                        set.insert(*location);
                    }
                }
            });

            match set.len().cmp(&2) {
                Ordering::Less => 0,
                Ordering::Equal => {
                    let mut set_iter = set.into_iter();

                    let first_number_location = set_iter.next().unwrap();
                    let first_number_row = matrix[first_number_location.row];
                    let first_number_str = &first_number_row[first_number_location.start..first_number_location.end];
                    let first_number = first_number_str.parse::<u32>().unwrap();

                    let second_number_location = set_iter.next().unwrap();
                    let second_number_row = matrix[second_number_location.row];
                    let second_number_str = &second_number_row[second_number_location.start..second_number_location.end];
                    let second_number = second_number_str.parse::<u32>().unwrap();

                    first_number * second_number
                },
                Ordering::Greater => panic!(
                    "More than two numbers found adjacent to asterisk at row {asterisk_row} col {asterisk_col}"
                ),
            }
        })
        .sum()
}

pub fn hashmap_locations_no_hashset_gear_ratios(mut input: &str) -> u32 {
    input = input.trim();

    let lines: Vec<&str> = input.lines().collect();

    let mut all_locations: HashMap<(u16, u16), LocationTuple> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        let mut previous_byte_offset = 0;
        for part_str in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
        {
            let substring = &line[previous_byte_offset..line.len()];
            let start = substring.find(part_str).unwrap() + previous_byte_offset;
            previous_byte_offset = start + part_str.len();

            let row = row as u16;
            let start = start as u16;
            let end = start + part_str.len() as u16;
            for col in start..end {
                all_locations.insert((row, col), (row, start, end));
            }
        }
    }

    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.match_indices('*')
                .map(move |(index, _)| (row as u16, index as u16))
        })
        .filter_map(|(asterisk_row, asterisk_col)| {
            let mut first: Option<LocationTuple> = None;
            let mut second: Option<LocationTuple> = None;

            let filtered_adjacent_locations = [
                (Some(asterisk_row), asterisk_col.checked_add(1)),
                (asterisk_row.checked_sub(1), asterisk_col.checked_add(1)),
                (asterisk_row.checked_sub(1), Some(asterisk_col)),
                (asterisk_row.checked_sub(1), asterisk_col.checked_sub(1)),
                (Some(asterisk_row), asterisk_col.checked_sub(1)),
                (asterisk_row.checked_add(1), asterisk_col.checked_sub(1)),
                (asterisk_row.checked_add(1), Some(asterisk_col)),
                (asterisk_row.checked_add(1), asterisk_col.checked_add(1)),
            ]
            .into_iter()
            .filter_map(|(row, col)| all_locations.get(&(row?, col?)));

            for &location in filtered_adjacent_locations {
                if first.is_none() {
                    first = Some(location);
                } else if first.unwrap() != location {
                    second = Some(location);
                    break;
                }
            }

            Some(get_number_from_lines(&lines, first?) * get_number_from_lines(&lines, second?))
        })
        .sum()
}

#[inline(always)]
fn get_number_from_lines(lines: &[&str], location: LocationTuple) -> u32 {
    let (row, start, end) = location;
    lines[row as usize][(start as usize)..(end as usize)]
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::INPUT;

    #[test]
    fn first_impl_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
*617......
.....+.58*
..592.....
......755.
...$.*....
.664.598..
";

        let actual = first_impl_gear_ratios(input);

        assert_eq!(actual, 467835);
    }

    #[test]
    fn first_impl_func_real_input() {
        let actual: u32 = hashmap_locations_no_hashset_gear_ratios(INPUT);

        assert_eq!(actual, 80253814);
    }

    #[test]
    fn hashmap_locations_no_hashset_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
*617......
.....+.58*
..592.....
......755.
...$.*....
.664.598..
";

        let actual = first_impl_gear_ratios(input);

        assert_eq!(actual, 467835);
    }

    #[test]
    fn hashmap_locations_no_hashset_real_input() {
        let actual: u32 = hashmap_locations_no_hashset_gear_ratios(INPUT);

        assert_eq!(actual, 80253814);
    }
}
