use regex::Regex;

advent_of_code::solution!(13);

type ClawMachine = Vec<((i128,i128,i128),(i128,i128,i128))>;

fn parse(input: &str) -> ClawMachine {
    let regex = Regex::new(r"X[\+|=](\d+), Y[\+|=](\d+)").unwrap();

    let matches: Vec<i128> = regex
        .captures_iter(input)
        .flat_map(|captures| {
            vec![captures.get(1).unwrap().as_str().parse::<i128>().unwrap(), captures.get(2).unwrap().as_str().parse::<i128>().unwrap()]
        })
        .collect();

    matches
        .chunks(6)
        .map(|c| ((c[0], c[2], c[4]), (c[1], c[3], c[5])))
        .collect()        
}

fn calc_tokens(input: ClawMachine) -> i128 {
    let mut tokens = 0;
    for (x,y) in input {
        let num = y.2*x.0 - x.2*y.0;
        let den = y.1*x.0 - x.1*y.0;
        
        if num % den != 0 {
            continue;
        }

        let b_presses = num/den;
        let a_presses = (x.2 - b_presses*x.1)/x.0;

        tokens += 3 * a_presses + b_presses;
    }

    tokens
}

pub fn part_one(input: &str) -> Option<i128> {
    Some(calc_tokens(parse(input)))
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut claw_values = parse(input);

    claw_values
        .iter_mut()
        .for_each(|((_, _, a),(_, _, b))| {
            *a += 10000000000000;
            *b += 10000000000000;
        });

    Some(calc_tokens(claw_values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result: Option<i128> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
