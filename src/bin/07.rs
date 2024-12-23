advent_of_code::solution!(7);

fn generate_permutations(operators: &[&str], length: usize) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    backtrack(
        operators,
        length,
        &mut Vec::with_capacity(length),
        &mut results,
    );
    results
}

fn backtrack(
    operators: &[&str],
    length: usize,
    current: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    if current.len() == length {
        results.push(current.clone());
        return;
    }

    for &s in operators {
        current.push(s.to_string());
        backtrack(operators, length, current, results);
        current.pop();
    }
}

fn calibrate(input: &str, operators: &[&str]) -> u128 {
    let mut total = 0;

    for line in input.lines() {
        let equation: Vec<u128> = line
            .split([':', ' '])
            .filter_map(|s| s.parse::<u128>().ok())
            .collect();

        let permutations = generate_permutations(operators, equation.len() - 2);

        for (i, _) in permutations.iter().enumerate() {
            let mut result = equation[1];

            for (j, &val) in equation.iter().skip(2).enumerate() {
                match permutations[i][j].as_str() {
                    "+" => result += val,
                    "*" => result *= val,
                    "||" => result = (result.to_string() + &val.to_string()).parse().unwrap(),
                    _ => panic!(),
                }
            }

            if result == equation[0] {
                total += result;
                break;
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(calibrate(input, &["+", "*"]))
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(calibrate(input, &["+", "*", "||"]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
