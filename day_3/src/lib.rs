pub mod input;

use ndarray::{Array2, ArrayView1, Axis};
use regex::Regex;

pub fn get_all_part_numbers(mut input: &str) -> u32 {
    let regex = Regex::new(r"(\d+)").unwrap();
    input = input.trim();
    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap_or(input.len());
    let vec: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let matrix: Array2<char> = Array2::from_shape_vec((row_len, col_len), vec).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| regex.find_iter(line).map(move |capture| (row, capture)))
        .map(|(row, capture)| {
            let col = capture.start();
            let len = capture.len();
            let symbol = |c: &char| !c.is_ascii_digit() && *c != '.';
            let skip = row.saturating_sub(1);
            let take = if row == 0 { 2 } else { 3 };

            let row_above = find_matrix_row(row.checked_sub(1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let row_below = find_matrix_row(Some(row + 1), &matrix)
                .map(|row| row.iter().skip(col).take(len).any(symbol))
                .unwrap_or(false);
            let col_left = find_matrix_col(col.checked_sub(1), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);
            let col_right = find_matrix_col(Some(col + len), &matrix)
                .map(|col| col.iter().skip(skip).take(take).any(symbol))
                .unwrap_or(false);

            if !(row_above || row_below || col_left || col_right) {
                return 0;
            }
            capture.as_str().parse::<u32>().unwrap()
        })
        .sum()
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

        assert_eq!(actual, 4363);
    }

    #[test]
    fn test_real_input() {
        let input = INPUT;

        let actual: u32 = get_all_part_numbers(input);

        assert_eq!(actual, 530495);
    }
}
