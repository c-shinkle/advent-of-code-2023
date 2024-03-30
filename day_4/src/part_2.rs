use std::iter::repeat;

pub fn part_2_impr(cards: &str) -> u32 {
    let number_of_cards = cards.trim().lines().count();
    let mut card_counts: Vec<u32> = repeat(1).take(number_of_cards).collect();
    for (card_index, line) in cards.trim().lines().enumerate() {
        let colon_pos = line.find(':').unwrap();
        let bar_pos = line.find('|').unwrap();
        let winning_numbers: Vec<u8> = line[(colon_pos + 1)..bar_pos]
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<u8>().unwrap())
            .collect();
        let matches = line[(bar_pos + 1)..]
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<u8>().unwrap())
            .filter(|number| winning_numbers.contains(number))
            .count();
        for offset in 1..=matches {
            card_counts[card_index + offset] += card_counts[card_index];
        }
    }
    card_counts.into_iter().sum()
}

pub fn part_2_func(cards: &str) -> u32 {
    let number_of_cards = cards.trim().lines().count();
    cards
        .trim()
        .lines()
        .map(|line| {
            let colon_pos = line.find(':').unwrap();
            let bar_pos = line.find('|').unwrap();
            let winning_numbers: Vec<u8> = line[(colon_pos + 1)..bar_pos]
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<u8>().unwrap())
                .collect();
            line[(bar_pos + 1)..]
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<u8>().unwrap())
                .filter(|number| winning_numbers.contains(number))
                .count()
        })
        .enumerate()
        .fold(
            repeat(1).take(number_of_cards).collect::<Vec<u32>>(),
            |acc, (card_index, matches)| {
                let beginning = acc.iter().take(card_index + 1).copied();
                let middle = (1..=matches).map(|offset| acc[card_index] + acc[card_index + offset]);
                let end = acc.iter().skip(card_index + matches + 1).copied();
                beginning.chain(middle).chain(end).collect()
            },
        )
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn impr_it_works() {
        let given = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = part_2_impr(given);
        assert_eq!(actual, 30);
    }

    #[test]
    fn func_it_works() {
        let given = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = part_2_func(given);
        assert_eq!(actual, 30);
    }

    #[test]
    fn impr_puzzle_input() {
        let given = input::INPUT;
        let actual = part_2_impr(given);
        assert_eq!(actual, 11024379);
    }

    #[test]
    fn func_puzzle_input() {
        let given = input::INPUT;
        let actual = part_2_func(given);
        assert_eq!(actual, 11024379);
    }
}
