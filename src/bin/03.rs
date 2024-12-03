use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: u32 = regex
        .captures_iter(input)
        .filter_map(|captures| {
            Some((
                captures.get(1)?.as_str().parse::<u32>().unwrap(),
                captures.get(2)?.as_str().parse::<u32>().unwrap(),
            ))
        })
        .map(|pair| pair.0 * pair.1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let result= regex
        .captures_iter(input)
        .filter_map(|matches| {
            if matches.get(1).is_none() {
                Some(vec![matches.get(0)?.as_str()])
            } else {
                Some(vec![
                    matches.get(1)?.as_str(),
                    matches.get(2)?.as_str(),
                ])
            }
        })
        .collect_vec();

    let mut skip = false;
    let mut sum = 0; 
    for item in result {
        match item.as_slice() {
            &["do()"] => skip = false,
            &["don't()"] => skip = true,
            mul => if !skip {
                sum += mul[0].parse::<u32>().unwrap() * mul[1].parse::<u32>().unwrap();
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
