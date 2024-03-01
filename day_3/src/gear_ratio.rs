use std::collections::HashMap;

type Location = (u16, u16, u16);

pub fn get_all_gear_ratios_func(mut input: &str) -> u32 {
    input = input.trim();

    let lines: Vec<&str> = input.lines().collect();

    let mut all_locations: HashMap<(u16, u16), Location> = HashMap::new();
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
            .filter_map(|(row, col)| all_locations.get(&(row?, col?)));

            for &location in filtered_adjacent_locations {
                if first.is_none() {
                    first = Some(location);
                } else if unsafe { first.unwrap_unchecked() } != location {
                    second = Some(location);
                    break;
                }
            }

            if first.is_none() || second.is_none() {
                return None;
            }

            Some(
                get_number_from_lines(&lines, first.unwrap())
                    * get_number_from_lines(&lines, second.unwrap()),
            )
        })
        .sum()
}

#[inline(always)]
fn get_number_from_lines(lines: &[&str], location: Location) -> u32 {
    let (row, start, end) = location;
    lines[row as usize][(start as usize)..(end as usize)]
        .parse::<u32>()
        .unwrap()
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
