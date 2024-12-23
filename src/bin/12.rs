use rust_utils::utils::pad;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(12);

type Perimiters = Vec<HashMap<(i32, i32), Vec<(usize, usize)>>>;

fn add(lhs: (usize, usize), rhs: (i32, i32)) -> (usize, usize) {
    (
        (lhs.0 as i32 + rhs.0) as usize,
        (lhs.1 as i32 + rhs.1) as usize,
    )
}

fn get_areas_perimiters(input: &str) -> (Vec<usize>, Perimiters) {
    let grid: Vec<Vec<char>> = pad(input, '.');

    let width = grid[0].len();
    let height = grid.len();

    let mut areas = Vec::new();
    let mut perimiters = Vec::new();
    let mut unique = HashSet::new();

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            if unique.contains(&(i, j)) {
                continue;
            }

            areas.push(0);
            perimiters.push(HashMap::new());
            let mut queue = VecDeque::new();
            queue.push_back((i, j));

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();

                if unique.contains(&current) {
                    continue;
                }

                unique.insert(current);
                *areas.last_mut().unwrap() += 1;

                for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let next = add(current, dir);

                    if grid[next.1][next.0] == grid[j][i] {
                        queue.push_back(next);
                    } else {
                        perimiters
                            .last_mut()
                            .unwrap()
                            .entry(dir)
                            .or_insert_with(Vec::new)
                            .push(next);
                    }
                }
            }
        }
    }

    (areas, perimiters)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (areas, perimiters) = get_areas_perimiters(input);

    let mut price = 0;
    for i in 0..areas.len() {
        price += perimiters[i].values().flatten().count() * areas[i];
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (areas, perimiters) = get_areas_perimiters(input);

    let mut price = 0;
    for i in 0..areas.len() {
        let mut sides = 4;
        for dir in [(1, 0), (-1, 0)] {
            let mut borders = perimiters[i][&dir].clone();
            borders.sort();
            for j in 1..borders.len() {
                if borders[j].0 != borders[j - 1].0 || borders[j].1 != borders[j - 1].1 + 1 {
                    sides += 1;
                }
            }
        }

        for dir in [(0, 1), (0, -1)] {
            let mut borders = perimiters[i][&dir].clone();
            borders.sort_by_key(|&(x, y)| (y, x));

            for j in 1..borders.len() {
                if borders[j].1 != borders[j - 1].1 || borders[j].0 != borders[j - 1].0 + 1 {
                    sides += 1;
                }
            }
        }

        price += sides * areas[i];
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
