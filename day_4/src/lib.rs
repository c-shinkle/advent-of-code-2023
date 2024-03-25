pub mod input;

pub fn part_1_impr(cards: &str) -> u32 {
    let mut sum = 0;
    for line in cards.trim().lines() {
        let colon_pos = line.find(':').unwrap();
        let bar_pos = line.find('|').unwrap();
        let winning_numbers: Vec<u32> = line[(colon_pos + 1)..bar_pos]
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<u32>().unwrap())
            .collect();
        let mut line_value: Option<u32> = None;
        let numbers = line[(bar_pos + 1)..]
            .split_ascii_whitespace()
            .map(|number_str| number_str.parse::<u32>().unwrap());
        for number in numbers {
            line_value = match (winning_numbers.contains(&number), line_value) {
                (false, _) => line_value,
                (true, None) => Some(1),
                (true, Some(previous_value)) => Some(previous_value * 2),
            }
        }
        sum += line_value.unwrap_or(0);
    }
    sum
}

pub fn part_1_func(cards: &str) -> u32 {
    let lines = cards.trim().lines();
    lines
        .map(|line| {
            let colon_pos = line.find(':').unwrap();
            let bar_pos = line.find('|').unwrap();
            let winning_numbers: Vec<u32> = line[(colon_pos + 1)..bar_pos]
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<u32>().unwrap())
                .collect();
            line[(bar_pos + 1)..]
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<u32>().unwrap())
                .fold(None, |acc, number| {
                    match (winning_numbers.contains(&number), acc) {
                        (false, _) => acc,
                        (true, None) => Some(1),
                        (true, Some(previous_value)) => Some(previous_value * 2),
                    }
                })
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impr_it_works() {
        let given = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = part_1_impr(given);
        assert_eq!(actual, 13);
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
        let actual = part_1_impr(given);
        assert_eq!(actual, 13);
    }

    #[test]
    fn impr_puzzle_input() {
        let given = input::INPUT;
        let actual = part_1_impr(given);
        assert_eq!(actual, 21485);
    }

    #[test]
    fn func_puzzle_input() {
        let given = input::INPUT;
        let actual = part_1_func(given);
        assert_eq!(actual, 21485);
    }
}
