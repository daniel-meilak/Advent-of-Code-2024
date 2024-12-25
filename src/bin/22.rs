use std::collections::HashMap;

advent_of_code::solution!(22);

fn next_secret(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret %= 16777216;

    secret ^= secret / 32;
    secret %= 16777216;

    secret ^= secret * 2048;
    secret %= 16777216;

    secret
}

fn nth_secret(mut secret: i64, n: usize) -> i64 {
    for _ in 0..n {
        secret = next_secret(secret);
    }

    secret
}

fn fill_secrets(secret: i64, n: usize) -> Vec<i64> {
    std::iter::successors(Some(secret), |&prev| Some(next_secret(prev)))
        .take(n)
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let sum = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|secret| nth_secret(secret, 2000))
        .sum();
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let future_secrets: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|secret| fill_secrets(secret, 2000) )
        .collect();

    let secret_diffs: Vec<Vec<i64>> = future_secrets
        .iter()
        .map(|secrets| secrets.windows(2).map(|v| (v[1] % 10) - (v[0] % 10)).collect())
        .collect();

    let future_sequences: Vec<HashMap<_,_>> = {
        secret_diffs
            .iter()
            .enumerate()
            .map(|(i, diffs)| {
                let mut sequence = HashMap::new();
                diffs
                    .windows(4)
                    .enumerate()
                    .for_each(|(j, v)| { sequence.entry(v.to_owned()).or_insert(future_secrets[i][j+4] % 10); });
                sequence
            })
            .collect()
    };

    let mut best_sequences = HashMap::new();
    for sequences in future_sequences {
        for (sequence, bananas) in sequences {
            *best_sequences.entry(sequence).or_insert(0) += bananas;
        } 
    }

    let best = best_sequences.into_iter().max_by_key(|&(_, value)| value).unwrap();
    
    Some(best.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
