use rust_utils::utils::{rotate, split_2d_by_regex, to_2d_numeric};
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(data: &str) -> Option<i32> {
    let mut input: Vec<Vec<i32>> = to_2d_numeric(split_2d_by_regex(data, "\\s+"));

    input = rotate(&input);

    input.iter_mut().for_each(|row| row.sort());

    let mut sum = 0;
    for i in 0..input[0].len() {
        sum += (input[0][i] - input[1][i]).abs();
    }

    Some(sum)
}

pub fn part_two(data: &str) -> Option<i32> {
    let mut input: Vec<Vec<i32>> = to_2d_numeric(split_2d_by_regex(data, "\\s+"));

    input = rotate(&input);

    input.iter_mut().for_each(|row| row.sort());

    let mut occurrences = HashMap::new();

    for i in &input[1] {
        *occurrences.entry(i).or_insert(0) += 1;
    }

    let mut sum = 0;
    for i in &input[0] {
        sum += i * occurrences.get(&i).unwrap_or(&0);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
