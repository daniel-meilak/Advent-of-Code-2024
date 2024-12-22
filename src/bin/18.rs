use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

#[cfg(test)]
const MAX_WIDTH: i32 = 6;
#[cfg(test)]
const MAX_HEIGHT: i32 = 6;
#[cfg(test)]
const BYTES: usize = 12;

#[cfg(not(test))]
const MAX_WIDTH: i32 = 70;
#[cfg(not(test))]
const MAX_HEIGHT: i32 = 70;
#[cfg(not(test))]
const BYTES: usize = 1024;

type Point = (i32, i32);

fn in_bounds(pos: Point) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 <= MAX_WIDTH && pos.1 <= MAX_HEIGHT
}

fn get_neighbours(pos: Point, grid: &HashSet<Point>, unique: &HashSet<Point>) -> Vec<Point> {
    let mut neighbours = Vec::new();
    for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if in_bounds(next) && !grid.contains(&next) && !unique.contains(&next) {
            neighbours.push(next);
        }
    } 

    neighbours
}

fn fill_grid(input: &str) -> (Vec<Point>, HashSet<Point>) {
    let mut grid= HashSet::new();

    let byte_positions: Vec<Point> = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    for byte in byte_positions[0..BYTES].iter() {
        grid.insert(*byte);
    }

    (byte_positions, grid)
}

fn get_path(grid: &HashSet<Point>) -> HashSet<Point> {
    let start = (0,0);
    let end = (MAX_WIDTH, MAX_HEIGHT);

    let mut unique = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, HashSet::from([start])));

    while !queue.is_empty() {
        let (pos, path) = queue.pop_front().unwrap();

        if pos == end {
            return path;
        }

        for neighbour in get_neighbours(pos, grid, &unique) {
            let mut new_path = path.clone();
            new_path.insert(neighbour);
            queue.push_back((neighbour, new_path));
            unique.insert(neighbour);
        }
    }

    HashSet::new()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, grid) = fill_grid(input);

    let path = get_path(&grid);

    Some(path.len() - 1)
}

pub fn part_two(input: &str) -> Option<String> {
    let (byte_positions, mut grid) = fill_grid(input);

    let mut path = get_path(&grid);
    
    let mut impass = (0, 0);
    for byte in byte_positions[BYTES..].iter() {
        grid.insert(*byte);

        if !path.contains(byte) {
            continue;
        }

        path = get_path(&grid);

        if path.is_empty() {
            impass = *byte;
            break;
        }
    }

    Some(format!("{},{}", impass.0, impass.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
