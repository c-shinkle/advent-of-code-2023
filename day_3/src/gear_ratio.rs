use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    start: u16,
    end: u16,
    row: u16,
}

pub fn get_all_gear_ratios_func(mut input: &str) -> u32 {
    input = input.trim();
    let mut all_locations = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        let mut previous_byte_offset = 0;
        for part_str in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
        {
            let substring = &line[previous_byte_offset..line.len()];
            let start = substring.find(part_str).unwrap() + previous_byte_offset;
            previous_byte_offset = start + part_str.len();

            let start = start as u16;
            let end = start + part_str.len() as u16;
            let row = row as u16;
            let location = Location { start, end, row };
            for i in start..end {
                all_locations.insert((row, i), location);
            }
        }
    }

    let rows: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for (asterisk_row, asterisk_col) in rows.iter().enumerate().flat_map(|(row, line)| {
        line.match_indices('*')
            .map(move |(index, _)| (row as u16, index as u16))
    }) {
        let mut first: Option<Location> = None;
        let mut second: Option<Location> = None;

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
        .filter_map(|(row, col)| all_locations.get(&(row?, col?)))
        .copied();

        for location in filtered_adjacent_locations {
            if first.is_none() {
                first = Some(location);
            } else if first.unwrap() != location {
                second = Some(location);
                break;
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            let first_number = rows[first.row as usize]
                [(first.start as usize)..(first.end as usize)]
                .parse::<u32>()
                .unwrap();

            let second_number = rows[second.row as usize]
                [(second.start as usize)..(second.end as usize)]
                .parse::<u32>()
                .unwrap();

            sum += first_number * second_number;
        }
    }
    sum
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
