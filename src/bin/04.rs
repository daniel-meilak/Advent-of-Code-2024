advent_of_code::solution!(4);

fn check_xmas(i: usize, grid: &[char], gap: usize) -> bool {
    if i+3*gap+3 >= grid.len() { return false }
    let sequence = [grid[i], grid[i+gap+1], grid[i+2*gap+2], grid[i+3*gap+3]];
    let valid_patterns = [['X', 'M', 'A', 'S'], ['S', 'A', 'M', 'X']];

    valid_patterns.contains(&sequence)
}

fn check_cross(i: usize, grid: &[char], gap: usize) -> bool {
    if i+2*gap+4 >= grid.len() { return false }
    let sequence = [grid[i], grid[i+2], grid[i+gap+2], grid[i+2*gap+2], grid[i+2*gap+4]];
    let valid_patterns = [['M', 'S', 'A', 'M', 'S'], ['M', 'M', 'A', 'S', 'S'], ['S', 'M', 'A', 'S', 'M'], ['S', 'S', 'A','M', 'M']];
    
    valid_patterns.contains(&sequence)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<char> = input.chars().collect();
    let width = input.find('\n').unwrap();
    let gaps = vec![0, width - 1, width, width +1];

    let mut count = 0;
    for j in gaps {
        for i in 0..input.len() {
            count += check_xmas(i, &grid, j) as u32;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<char> = input.chars().collect();
    let width = input.find('\n').unwrap();

    let mut count = 0;
    for i in 0..input.len() {
        count += check_cross(i, &grid, width) as u32;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
