pub mod input;

use ndarray::{Array2, ArrayView1, Axis};
use regex::Regex;

pub fn get_all_part_numbers(mut input: &str) -> Vec<u32> {
    input = input.trim();
    let mut part_numbers: Vec<u32> = Vec::new();

    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap_or(input.len());
    let vec: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();

    let matrix: Array2<char> = Array2::from_shape_vec((row_len, col_len), vec).unwrap();

    let regex = Regex::new(r"(\d+)").unwrap();

    for (row, line) in input.lines().enumerate() {
        for capture in regex.find_iter(line) {
            let col = capture.start();
            let number_str = capture.as_str();

            let len = number_str.len();
            let symbol = |c: &char| !c.is_ascii_digit() && *c != '.';
            let mut was_symbol_found = false;
            // row above
            if let Some(row) = find_matrix_row(row.checked_sub(1), &matrix) {
                was_symbol_found |= row.iter().skip(col).take(len).any(symbol);
            }
            // row below
            if let Some(row) = find_matrix_row(Some(row + 1), &matrix) {
                was_symbol_found |= row.iter().skip(col).take(len).any(symbol);
            }
            // column to the left
            let skip = row.saturating_sub(1);
            let take = if row == 0 { 2 } else { 3 };
            if let Some(col) = find_matrix_col(col.checked_sub(1), &matrix) {
                was_symbol_found |= col.iter().skip(skip).take(take).any(symbol);
            }
            // column to the right
            if let Some(col) = find_matrix_col(Some(col + len), &matrix) {
                was_symbol_found |= col.iter().skip(skip).take(take).any(symbol);
            }
            if was_symbol_found {
                part_numbers.push(number_str.parse::<u32>().unwrap());
            }
        }
    }

    part_numbers
}

fn find_matrix_row(i: Option<usize>, matrix: &Array2<char>) -> Option<ArrayView1<char>> {
    if i? >= matrix.len_of(Axis(0)) {
        return None;
    }
    Some(matrix.row(i?))
}

fn find_matrix_col(i: Option<usize>, matrix: &Array2<char>) -> Option<ArrayView1<char>> {
    if i? >= matrix.len_of(Axis(1)) {
        return None;
    }
    Some(matrix.column(i?))
}

#[cfg(test)]
mod test {
    use crate::input::INPUT;

    use super::*;
    #[test]
    fn test_all_edge_cases() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
......&...
.2...2....
";

        let actual = get_all_part_numbers(input);

        assert_eq!(actual, vec![467, 35, 633, 617, 592, 755, 664, 598, 2]);
    }

    #[test]
    fn test_real_input() {
        let input = INPUT;

        let actual: u32 = get_all_part_numbers(input).iter().sum();

        assert_eq!(actual, 530495);
    }
}
