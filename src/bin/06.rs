use core::panic;
use rust_utils::utils::pad;
use std::collections::HashSet;

advent_of_code::solution!(6);

fn turn_right(dir: &char) -> char {
    match *dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!(),
    }
}

fn next_step(
    orientation: &mut char,
    pos: &mut (usize, usize),
    grid: &[Vec<char>],
    turning_points: &mut HashSet<((usize, usize), char)>,
) -> (char, bool) {
    let mut next = *pos;
    match orientation {
        '^' => next.1 -= 1,
        'v' => next.1 += 1,
        '<' => next.0 -= 1,
        '>' => next.0 += 1,
        _ => panic!(),
    }

    let cell = grid[next.1][next.0];
    let mut visited = false;
    if cell == '#' {
        visited = !turning_points.insert((*pos, *orientation));
        *orientation = turn_right(orientation);
    } else {
        *pos = next;
    }

    (cell, visited)
}

fn start(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(j, row)| row.iter().position(|&c| c == '^').map(|i| (i, j)))
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = pad(input, 'X');

    let mut pos = start(&grid);

    let mut unique = HashSet::new();
    unique.insert(pos);

    let mut orientation = '^';
    while next_step(&mut orientation, &mut pos, &grid, &mut HashSet::new()).0 != 'X' {
        unique.insert(pos);
    }

    Some(unique.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = pad(input, 'X');

    let begin = start(&grid);
    let mut loops = 0;

    for j in 1..grid.len() - 1 {
        for i in 1..grid[0].len() - 1 {
            if grid[j][i] == '.' {
                grid[j][i] = '#'
            } else {
                continue;
            }

            let mut pos = begin;
            let mut orientation = '^';
            let mut turning_points = HashSet::new();

            loop {
                let result = next_step(&mut orientation, &mut pos, &grid, &mut turning_points);
                if result.0 == 'X' {
                    break;
                } else if result.1 {
                    loops += 1;
                    break;
                }
            }

            grid[j][i] = '.';
        }
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
