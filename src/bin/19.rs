use cached::proc_macro::cached;

advent_of_code::solution!(19);

#[cached]
fn ways_to_build(design: String, patterns: Vec<String>) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;
    for pattern in patterns.iter() {
        if let Some(left_over) = design.strip_prefix(pattern) {
            count += ways_to_build(left_over.to_owned(), patterns.clone());
        }
    }

    count
}

fn parse(input: &str) -> (Vec<String>, String) {
    let (pattern_list, design_list) = input.split_once("\n\n").unwrap();

    let patterns: Vec<String> = pattern_list.split(", ").map(|s| s.to_owned()).collect();

    (patterns, design_list.to_owned())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns, designs) = parse(input);

    let count = designs
        .lines()
        .filter(|&design| ways_to_build(design.to_owned(), patterns.clone()) > 0)
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, designs) = parse(input);

    let count = designs
        .lines()
        .map(|design| ways_to_build(design.to_owned(), patterns.clone()))
        .sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
