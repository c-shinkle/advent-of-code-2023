pub mod input;

use core::char;
use std::{
    cmp::{max_by_key, min_by_key},
    collections::HashMap,
};

type IndexValue = (usize, u32);

pub fn find_number(line: &str) -> Option<u32> {
    let mut word_lookup: HashMap<&str, u32> = HashMap::new();
    word_lookup.insert("zero", 0);
    word_lookup.insert("one", 1);
    word_lookup.insert("two", 2);
    word_lookup.insert("three", 3);
    word_lookup.insert("four", 4);
    word_lookup.insert("five", 5);
    word_lookup.insert("six", 6);
    word_lookup.insert("seven", 7);
    word_lookup.insert("eight", 8);
    word_lookup.insert("nine", 9);

    let chars: Vec<char> = line.chars().collect();
    let first_digit_with_index = chars
        .iter()
        .enumerate()
        .find_map(|(index, char)| char.to_digit(10).map(|value| (index, value)));
    let first_word_with_index: Option<IndexValue> = word_lookup
        .keys()
        .map(|word| (line.find(word), word))
        .filter_map(|(index, word)| index.map(|index| (index, *word_lookup.get(word).unwrap())))
        .min_by_key(|a| a.0);
    let first = match (first_digit_with_index, first_word_with_index) {
        (Some(first), Some(second)) => Some(min_by_key(first, second, |a| a.0).1),
        (Some((_, digit_value)), None) => Some(digit_value),
        (None, Some((_, word_value))) => Some(word_value),
        (None, None) => None,
    };

    let len = chars.len();
    let second_digit_with_index = chars
        .into_iter()
        .rev()
        .enumerate()
        .find_map(|(index, char)| char.to_digit(10).map(|value| (len - 1 - index, value)));
    let second_word_with_index: Option<IndexValue> = word_lookup
        .keys()
        .map(|word| (line.rfind(word), word))
        .filter_map(|(index, word)| index.map(|index| (index, *word_lookup.get(word).unwrap())))
        .max_by_key(|a| a.0);
    let second = match (second_digit_with_index, second_word_with_index) {
        (Some(first), Some(second)) => Some(max_by_key(first, second, |a| a.0).1),
        (Some((_, digit_value)), None) => Some(digit_value),
        (None, Some((_, word_value))) => Some(word_value),
        (None, None) => None,
    };

    match (first, second) {
        (Some(first), Some(second)) => Some(first * 10 + second),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two1nine() {
        assert_eq!(Some(29), find_number("two1nine"));
    }

    #[test]
    fn test_7fjqhrhsevenlbtwoninevnmct2() {
        assert_eq!(Some(72), find_number("7fjqhrhsevenlbtwoninevnmct2"));
    }
}
