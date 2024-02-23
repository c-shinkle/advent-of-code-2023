use std::cmp::Ordering;
use std::collections::HashSet;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    start: usize,
    end: usize,
    row: usize,
}

pub fn get_all_gear_ratios_func(mut input: &str) -> u32 {
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

#[cfg(test)]
mod test {
    use crate::input::INPUT;

    use super::*;
    #[test]
    fn gear_ratio_func_all_edge_cases() {
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

        let actual = get_all_gear_ratios_func(input);

        assert_eq!(actual, 467835);
    }

    #[test]
    fn gear_ratio_func_real_input() {
        let actual: u32 = get_all_gear_ratios_func(INPUT);

        assert_eq!(actual, 80253814);
    }
}
