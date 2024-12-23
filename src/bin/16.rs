use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

fn score_and_paths(input: &str) -> (usize, Vec<Vec<(usize, usize)>>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = (1, grid.len() - 2);
    let end = (grid[0].len() - 2, 1);

    let mut unique = HashMap::new();
    let mut queue = VecDeque::from([(vec![start], (1, 0), 0)]);

    let mut best_paths = Vec::new();
    let mut min_score = usize::MAX;

    while !queue.is_empty() {
        let (path, dir, score) = queue.pop_front().unwrap();
        let pos = *path.last().unwrap();

        if score > min_score {
            continue;
        }

        if unique.contains_key(&(pos, dir)) && unique[&(pos, dir)] < score {
            continue;
        } else {
            unique.insert((pos, dir), score);
        }

        if pos == end {
            if score < min_score {
                best_paths.clear();
                min_score = score;
            }

            best_paths.push(path.clone());
        }

        for new_dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (
                (pos.0 as i32 + new_dir.0) as usize,
                (pos.1 as i32 + new_dir.1) as usize,
            );

            if grid[next.1][next.0] == '#' {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);
            if new_dir.0 * dir.0 + new_dir.1 * dir.1 == 1 {
                queue.push_back((new_path, new_dir, score + 1));
            } else {
                queue.push_back((new_path, new_dir, score + 1001));
            }
        }
    }

    (min_score, best_paths)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (score, _) = score_and_paths(input);

    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, paths) = score_and_paths(input);

    let unique: HashSet<_> = paths.into_iter().flatten().collect();

    Some(unique.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
