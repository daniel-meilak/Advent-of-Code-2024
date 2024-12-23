use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[cfg(test)]
const TIME_SAVE: usize = 70;

#[cfg(not(test))]
const TIME_SAVE: usize = 100;

type Point = (usize, usize);

fn find_char(c: char, grid: &str) -> Point {
    let pos = grid.find(c).unwrap();
    let width = grid.find('\n').unwrap() + 1;

    (pos % width, pos / width)
}

fn get_neighbours(pos: Point, step: i32, grid: &[Vec<char>]) -> Vec<Point> {
    let mut neighbours = Vec::new();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    for dx in -step..=step {
        let dy = step - dx.abs();

        neighbours.push((pos.0 as i32 + dx, pos.1 as i32 + dy));
        if dy != 0 {
            neighbours.push((pos.0 as i32 + dx, pos.1 as i32 - dy));
        }
    }

    neighbours
        .iter()
        .filter(|&next| {
            (0..width).contains(&next.0)
                && (0..height).contains(&next.1)
                && grid[next.1 as usize][next.0 as usize] != '#'
        })
        .map(|next| (next.0 as usize, next.1 as usize))
        .collect()
}

fn get_times(input: &str) -> (HashMap<Point, usize>, Vec<Vec<char>>) {
    let start = find_char('S', input);
    let end = find_char('E', input);

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut queue = VecDeque::new();
    let mut baseline = HashMap::new();

    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (pos, time) = queue.pop_front().unwrap();

        if let Some(best_time) = baseline.get(&pos) {
            if *best_time <= time {
                continue;
            }
        } else {
            baseline.insert(pos, time);
        }

        if pos == end {
            break;
        }

        queue.extend(
            get_neighbours(pos, 1, &grid)
                .into_iter()
                .map(|next| (next, time + 1)),
        );
    }

    (baseline, grid)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (baseline, grid) = get_times(input);

    let mut shortcuts = Vec::new();
    for (pos, time) in &baseline {
        for next in get_neighbours(*pos, 2, &grid) {
            if let Some(time_save) = baseline[&next].checked_sub(time + 2) {
                shortcuts.push(time_save);
            }
        }
    }

    let count = shortcuts.iter().filter(|&time| *time >= TIME_SAVE).count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (baseline, grid) = get_times(input);

    let mut shortcuts = Vec::new();
    for (pos, time) in &baseline {
        for i in 2..=20 {
            for next in get_neighbours(*pos, i, &grid) {
                if let Some(time_save) = baseline[&next].checked_sub(time + i as usize) {
                    shortcuts.push(time_save);
                }
            }
        }
    }

    let count = shortcuts.iter().filter(|&time| *time >= TIME_SAVE).count();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1418));
    }
}
