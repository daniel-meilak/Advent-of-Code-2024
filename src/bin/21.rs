use cached::proc_macro::cached;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(21);

const NUMERIC: &[&[char]] = &[
    &['#', '#', '#', '#', '#'],
    &['#', '7', '8', '9', '#'],
    &['#', '4', '5', '6', '#'],
    &['#', '1', '2', '3', '#'],
    &['#', '#', '0', 'A', '#'],
    &['#', '#', '#', '#', '#'],
];

const DIRECTIONAL: &[&[char]] = &[
    &['#', '#', '#', '#', '#'],
    &['#', '#', '^', 'A', '#'],
    &['#', '<', 'v', '>', '#'],
    &['#', '#', '#', '#', '#'],
];

fn find_button(button: char, pad: &[&[char]]) -> (usize, usize) {
    pad.iter()
        .enumerate()
        .find_map(|(j, row)| row.iter().position(|&c| c == button).map(|i| (i, j)))
        .unwrap()
}

#[cached]
fn get_nth_length(start: char, end: char, n: usize, numeric: bool) -> usize {
    if n == 0 {
        return 1;
    }

    let pad = if numeric { NUMERIC } else { DIRECTIONAL };

    let mut path = get_path(start, end, pad);
    path.insert(0, 'A');

    path.windows(2)
        .map(|v| get_nth_length(v[0], v[1], n - 1, false))
        .sum()
}

fn get_path(start: char, end: char, pad: &[&[char]]) -> Vec<char> {
    let start_button = find_button(start, pad);
    let end_button = find_button(end, pad);

    let mut paths: Vec<Vec<char>> = Vec::new();
    let mut unique = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_button, Vec::new()));
    unique.insert(start_button, 0);

    while let Some((pos, path)) = queue.pop_front() {
        if pos == end_button {
            if paths.is_empty() || path.len() == paths[0].len() {
                paths.push(path.clone());
            } else if path.len() < paths[0].len() {
                paths.clear();
                paths.push(path.clone());
            }
            continue;
        }

        for dir in [(-1, 0, '<'), (0, -1, '^'), (0, 1, 'v'), (1, 0, '>')] {
            let next = (
                (dir.0 + pos.0 as i32) as usize,
                (dir.1 + pos.1 as i32) as usize,
            );
            if pad[next.1][next.0] != '#'
                && (!unique.contains_key(&next) || path.len() <= *unique.get(&next).unwrap())
            {
                unique.insert(next, path.len());
                let mut new_path = path.clone();
                new_path.push(dir.2);
                queue.push_back((next, new_path));
            }
        }
    }

    paths.sort_by_key(|path| {
        path.iter()
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|pair| pair[0] != pair[1])
            .count()
    });
    paths[0].push('A');
    paths[0].to_owned()
}

fn complexity(input: &str, n: usize) -> usize {
    input
        .lines()
        .map(|line| {
            (
                line[0..line.len() - 1].parse::<usize>().unwrap(),
                format!("{}{}", 'A', line).chars().collect::<Vec<_>>(),
            )
        })
        .map(|(code, line)| {
            (
                code,
                line.windows(2)
                    .map(|v| get_nth_length(v[0], v[1], n, true))
                    .sum::<usize>(),
            )
        })
        .map(|(code, length)| code * length)
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(complexity(input, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(complexity(input, 26))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
