use cached::proc_macro::cached;

advent_of_code::solution!(11);

#[cached]
fn blink(stone: String, n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let result = match stone.as_str() {
        "0" => blink("1".to_string(), n - 1),
        _ if stone.len() % 2 == 0 => {
            let mid = stone.len()/2;
            let mut other_half = stone[mid..].trim_start_matches('0').to_string();
            other_half = if other_half.is_empty() {"0".to_string()} else { other_half };

            blink(stone[..mid].to_string(), n - 1) + blink(other_half, n - 1)
        },
        _ => blink((stone.parse::<u128>().unwrap() * 2024).to_string(), n - 1),
    };

    result
}

fn arrangement(input: &str, n: usize) -> usize {
    let stones: Vec<String> = input
        .split_ascii_whitespace()
        .map(|stone| stone.chars().collect())
        .collect();
    
    let result = stones
        .iter()
        .map(|stone| blink(stone.clone(), n))
        .sum();
    
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(arrangement(input, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(arrangement(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
