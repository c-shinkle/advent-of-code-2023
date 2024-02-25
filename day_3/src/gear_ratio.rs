// use cond_utils::Between;
use std::collections::HashMap;

// #[derive(PartialEq, Eq, Hash, Clone, Copy)]
// struct Location {
//     start: u16,
//     end: u16,
//     row: u16,
// }

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

    let mut sum = 0;
    // let row_len = lines.len();
    // let col_len = lines[0].len();
    for (asterisk_row, asterisk_col) in lines.iter().enumerate().flat_map(|(row, line)| {
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
        .filter_map(|(row, col)| all_locations.get(&(row?, col?)));

        for &location in filtered_adjacent_locations {
            if first.is_none() {
                first = Some(location);
            } else if unsafe { first.unwrap_unchecked() } != location {
                second = Some(location);
                break;
            }
        }

        if let (Some((row1, start1, end1)), Some((row2, start2, end2))) = (first, second) {
            let first_number = lines[row1 as usize][(start1 as usize)..(end1 as usize)]
                .parse::<u32>()
                .unwrap();

            let second_number = lines[row2 as usize][(start2 as usize)..(end2 as usize)]
                .parse::<u32>()
                .unwrap();

            sum += first_number * second_number;
        }
    }
    sum
}

// fn get_adjacent_cells(row: u16, col: u16, row_len: u16, col_len: u16) -> Vec<RowCol> {
//     // in the middle
//     if row.between(0, row_len) && col.between(0, col_len) {
//         vec![
//             (row, col + 1),
//             (row - 1, col + 1),
//             (row - 1, col),
//             (row - 1, col - 1),
//             (row, col - 1),
//             (row + 1, col - 1),
//             (row + 1, col),
//             (row + 1, col + 1),
//         ]
//         // right edge
//     } else if row.between(0, row_len) && col == col_len - 1 {
//         vec![
//             (row - 1, col),
//             (row - 1, col - 1),
//             (row, col - 1),
//             (row + 1, col - 1),
//             (row + 1, col),
//         ]
//         // top right corner
//     } else if row == 0 && col == col_len - 1 {
//         vec![(row, col - 1), (row + 1, col - 1), (row + 1, col)]
//         // top edge
//     } else if row == 0 && col.between(0, col_len) {
//         vec![
//             (row, col + 1),
//             (row, col - 1),
//             (row + 1, col - 1),
//             (row + 1, col),
//             (row + 1, col + 1),
//         ]
//         // top left corner
//     } else if row == 0 && col == 0 {
//         vec![(row, col + 1), (row + 1, col), (row + 1, col + 1)]
//         // left edge
//     } else if row.between(0, row_len) && col == 0 {
//         vec![
//             (row, col + 1),
//             (row - 1, col + 1),
//             (row - 1, col),
//             (row + 1, col),
//             (row + 1, col + 1),
//         ]
//         // bottom left corner
//     } else if row == row_len - 1 && col == 0 {
//         vec![(row, col + 1), (row - 1, col + 1), (row - 1, col)]
//         // bottom edge
//     } else if row == row_len - 1 && col.between(0, col_len) {
//         vec![
//             (row, col + 1),
//             (row - 1, col + 1),
//             (row - 1, col),
//             (row - 1, col - 1),
//             (row, col - 1),
//         ]
//         // bottom right corner
//     } else if row == row_len - 1 && col == col_len - 1 {
//         vec![(row - 1, col), (row - 1, col - 1), (row, col - 1)]
//     } else {
//         vec![]
//     }
// }

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
