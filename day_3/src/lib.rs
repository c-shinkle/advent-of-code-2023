pub mod input;

use ndarray::{Array2, ArrayView1, Axis};

pub fn get_all_part_numbers(input: &str) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();

    let row_len = input.matches('\n').count() + 1;
    let col_len = input.find('\n').unwrap();
    let vec: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();

    let matrix: Array2<char> = Array2::from_shape_vec((row_len, col_len), vec).unwrap();

    for (row, line) in input.lines().enumerate() {
        part_numbers.extend(
            line.split(|c: char| c == '.' || !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .filter_map(|number_str| -> Option<u32> {
                    let len = number_str.len();
                    let col = line.find(number_str).unwrap();
                    let symbol = |c: &char| !c.is_ascii_digit() && *c != '.';
                    let was_symbol_found = 
                        // row above
                        find_matrix_row(row.checked_sub(1), &matrix)
                                .map(|row| row.iter().skip(col).take(len).any(symbol))
                                .unwrap_or(false) ||
                        // row below
                        find_matrix_row(Some(row + 1), &matrix)
                            .map(|row| row.iter().skip(col).take(len).any(symbol))
                            .unwrap_or(false) ||
                        // column to the left
                        find_matrix_col(col.checked_sub(1), &matrix)
                            .map(|col| col.iter().skip(row.saturating_sub(1)).take(3).any(symbol))
                            .unwrap_or(false) ||
                        // column to the right
                        find_matrix_col(Some(col + len), &matrix)
                            .map(|col| col.iter().skip(row.saturating_sub(1)).take(3).any(symbol))
                            .unwrap_or(false);
                    if !was_symbol_found {
                        return None;
                    }
                    Some(number_str.parse().unwrap())
                }),
        );
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
    use super::*;

    #[test]
    fn test_01() {
        let input = "
..*..
.234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_02() {
        let input = "
.....
.234.
..*..
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_03() {
        let input = "
.....
*234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_04() {
        let input = "
.....
.234*
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_05() {
        let input = "
*....
.234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_06() {
        let input = "
....*
.234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_07() {
        let input = "
.....
.234.
*....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_08() {
        let input = "
.....
.234.
....*
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }

    #[test]
    fn test_09() {
        let input = "
.....
.2!3.
.....
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![2, 3]);
    }

    #[test]
    fn test_10() {
        let input = "
.2!3.
.....
.....
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![2, 3]);
    }

    #[test]
    fn test_11() {
        let input = "
.....
.....
.....
.2!3.
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![2, 3]);
    }

    #[test]
    fn test_12() {
        let input = "
.....
2....
!....
3....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![2, 3]);
    }

    #[test]
    fn test_13() {
        let input = "
......
......
...*..
.12.3.
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![12, 3]);
    }
    #[test]
    fn test_14() {
        let input = "
.*...
.234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }
    #[test]
    fn test_15() {
        let input = "
...*.
.234.
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }
    #[test]
    fn test_16() {
        let input = "
.....
.234.
.*...
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }
    #[test]
    fn test_17() {
        let input = "
.....
.234.
...*.
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![234]);
    }
    #[test]
    fn test_18() {
        let input = "
.....
...12
..*..
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![12]);
    }
    #[test]
    fn test_19() {
        let input = "
.*...
...12
.....
";

        let actual = get_all_part_numbers(input.trim());

        assert_eq!(actual, vec![]);
    }
}
