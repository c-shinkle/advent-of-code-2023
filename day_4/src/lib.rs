pub mod input;

pub fn get_points_for_cards(cards: &str) -> usize {
    let mut sum = 0;

    for line in cards.trim().lines() {
        let colon_pos = line.chars().position(|c| c == ':').unwrap();
        let bar_pos = line.chars().position(|c| c == '|').unwrap();

        let winning_numbers: Vec<usize> = line[(colon_pos + 1)..bar_pos]
            .trim()
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<usize>().unwrap())
            .collect();

        let numbers: Vec<usize> = line[(bar_pos + 1)..]
            .trim()
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<usize>().unwrap())
            .collect();

        let mut line_value: Option<usize> = None;

        for number in numbers {
            if winning_numbers.contains(&number) {
                line_value = match line_value {
                    Some(previous_value) => Some(previous_value * 2),
                    None => Some(1),
                }
            }
        }

        sum += line_value.unwrap_or(0);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let given = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = get_points_for_cards(given);
        assert_eq!(actual, 13);
    }

    #[test]
    fn puzzle_input() {
        let given = input::INPUT;
        let actual = get_points_for_cards(given);
        assert_eq!(actual, 21485);
    }
}
