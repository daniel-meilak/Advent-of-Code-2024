advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    let data = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    data
}

fn safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report
        .windows(2)
        .map(|levels| levels[1] - levels[0])
        .collect();

    diffs.iter().all(|x| (1..4).contains(x)) || diffs.iter().all(|x| (-3..0).contains(x))
}

fn safe_with_tolerance(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut partial = report.to_vec();
        partial.remove(i);

        if safe(&partial) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse(input).iter().filter(|x| safe(x)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .iter()
            .filter(|x| safe_with_tolerance(x))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
