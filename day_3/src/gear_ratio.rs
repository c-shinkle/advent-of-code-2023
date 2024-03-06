use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use num_traits::PrimInt;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    start: usize,
    end: usize,
    row: usize,
}

type LocationTuple = (u16, u16, u16);

#[inline(always)]
fn parse_number(lines: &[&str], row: u16, start: u16, end: u16) -> u32 {
    lines[row as usize][(start as usize)..(end as usize)]
        .parse()
        .unwrap()
}

#[inline(always)]
fn parse_number_usize(lines: &[&str], row: usize, start: usize, end: usize) -> u32 {
    lines[row][start..end].parse().unwrap()
}

#[inline(always)]
fn get_neighbors<N: PrimInt>(row: N, col: N) -> [(Option<N>, Option<N>); 8] {
    let one = N::one();
    [
        (Some(row), col.checked_add(&one)),
        (row.checked_sub(&one), col.checked_add(&one)),
        (row.checked_sub(&one), Some(col)),
        (row.checked_sub(&one), col.checked_sub(&one)),
        (Some(row), col.checked_sub(&one)),
        (row.checked_add(&one), col.checked_sub(&one)),
        (row.checked_add(&one), Some(col)),
        (row.checked_add(&one), col.checked_add(&one)),
    ]
}

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
            get_neighbors(asterisk_row, asterisk_col)
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

                    let Location { row, start, end } = set_iter.next().unwrap();
                    let first_number = parse_number_usize(&matrix, row, start, end);
                    let Location { row, start, end } = set_iter.next().unwrap();
                    let second_number = parse_number_usize(&matrix, row, start, end);

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

    let mut sum = 0;
    for (asterisk_row, asterisk_col) in lines.iter().enumerate().flat_map(|(row, line)| {
        line.match_indices('*')
            .map(move |(index, _)| (row as u16, index as u16))
    }) {
        let mut first: Option<LocationTuple> = None;
        let mut second: Option<LocationTuple> = None;

        for row_col in get_neighbors(asterisk_row, asterisk_col) {
            if let (Some(row), Some(col)) = row_col {
                if let Some(&location) = all_locations.get(&(row, col)) {
                    if first.is_none() {
                        first = Some(location);
                    } else if first.unwrap() != location {
                        second = Some(location);
                        break;
                    }
                }
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            sum += parse_number(&lines, first.0, first.1, first.2)
                * parse_number(&lines, second.0, second.1, second.2);
        }
    }

    sum
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
