use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(10);

fn get_neighbours(
    pos: (usize, usize),
    width: usize,
    height: usize,
    grid: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut neighbours = Vec::new();

    if x + 1 < width {
        neighbours.push((x + 1, y));
    }
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y + 1 < height {
        neighbours.push((x, y + 1));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }

    neighbours
        .into_iter()
        .filter(|(x, y)| grid[*y][*x] as u8 == 1 + grid[pos.1][pos.0] as u8)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let start: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(i, c)| if c == &'0' { Some((i, j)) } else { None })
        })
        .collect();

    let mut total = 0;
    for pos in start {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([pos]);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            if grid[current.1][current.0] == '9' {
                total += 1;
                continue;
            }

            for neighbour in get_neighbours(current, width, height, &grid) {
                if visited.insert(neighbour) {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let start: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(i, c)| if c == &'0' { Some((i, j)) } else { None })
        })
        .collect();

    let mut total = 0;
    for pos in start {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([vec![pos]]);

        while !queue.is_empty() {
            let current_path = queue.pop_front().unwrap();
            let current = current_path.last().unwrap();

            if grid[current.1][current.0] == '9' {
                total += 1;
                continue;
            }

            for neighbour in get_neighbours(*current, width, height, &grid) {
                let mut path = current_path.clone();
                path.push(neighbour);

                if visited.insert(path.clone()) {
                    queue.push_back(path);
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
