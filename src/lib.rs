pub mod input;

use std::{
    cmp::{max_by_key, min_by_key},
    collections::HashMap,
};

pub fn find_number(line: &str) -> Option<u32> {
    let mut value_lookup = HashMap::new();
    value_lookup.insert("zero", 0);
    value_lookup.insert("one", 1);
    value_lookup.insert("two", 2);
    value_lookup.insert("three", 3);
    value_lookup.insert("four", 4);
    value_lookup.insert("five", 5);
    value_lookup.insert("six", 6);
    value_lookup.insert("seven", 7);
    value_lookup.insert("eight", 8);
    value_lookup.insert("nine", 9);

    let mut first_digit_with_index: Option<(usize, u32)> = None;
    for (i, char) in line.char_indices() {
        if let Some(digit) = char.to_digit(10) {
            if first_digit_with_index.is_none() || i < first_digit_with_index.unwrap().0 {
                first_digit_with_index = Some((i, digit));
            }
        }
    }
    let mut first_word_with_index: Option<(usize, u32)> = None;
    for (word, value) in value_lookup.iter() {
        if let Some(i) = line.find(word) {
            if first_word_with_index.is_none() || i < first_word_with_index.unwrap().0 {
                first_word_with_index = Some((i, *value))
            }
        }
    }
    let first = match (first_digit_with_index, first_word_with_index) {
        (Some(first), Some(second)) => Some(min_by_key(first, second, |a| a.0).1),
        (Some((_, digit_value)), None) => Some(digit_value),
        (None, Some((_, word_value))) => Some(word_value),
        (None, None) => None,
    };

    let second_digit_with_index = line
        .char_indices()
        .rev()
        .find_map(|(i, char)| char.to_digit(10).map(|value| (i, value)));
    let second_word_with_index = value_lookup
        .into_iter()
        .filter_map(|(word, value)| Some((line.rfind(word)?, value)))
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
    fn first_digit_with_second_digit() {
        assert_eq!(Some(2), find_number("0one2"));
    }

    #[test]
    fn first_digit_with_second_word() {
        assert_eq!(Some(35), find_number("3fourfive"));
    }

    #[test]
    fn first_word_with_second_digit() {
        assert_eq!(Some(68), find_number("six78"));
    }

    #[test]
    fn first_word_with_second_word() {
        let actual = find_number("nine0one");
        assert_eq!(Some(91), actual);
    }

    #[test]
    fn test_where_find_matters_for_first_word() {
        let actual = find_number("1twothreetwo");
        assert_eq!(Some(12), actual);
    }

    #[test]
    fn test_where_rfind_matters_for_second_word() {
        let actual = find_number("twothreetwo1");
        assert_eq!(Some(21), actual);
    }
}
