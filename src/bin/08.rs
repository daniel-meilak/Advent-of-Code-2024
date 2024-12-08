use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

type AddAntinodesFn = fn((isize, isize), (isize, isize), (isize, isize), &mut HashSet<(isize, isize)>);

fn in_bounds(size: (isize, isize), pos: (isize, isize)) -> bool {
    (0..size.0).contains(&(pos.0)) && (0..size.1).contains(&(pos.1))
}

fn add_antinodes(a: (isize, isize), b: (isize, isize), size: (isize, isize), antinodes: &mut HashSet<(isize, isize)>) {
    let diff= (b.0 - a.0, b.1 - a.1);
    if in_bounds(size, (a.0 - diff.0, a.1 - diff.1)) { antinodes.insert((a.0 - diff.0, a.1 - diff.1)); }
    if in_bounds(size, (b.0 + diff.0, b.1 + diff.1)) { antinodes.insert((b.0 + diff.0, b.1 + diff.1)); }
}

fn add_resonant_antinodes(a: (isize, isize), b: (isize, isize), size: (isize, isize), antinodes: &mut HashSet<(isize, isize)>) {
    let diff= (b.0 - a.0, b.1 - a.1);

    (0..)
        .map(|i| (a.0 - diff.0 * i, a.1 - diff.1 * i))
        .take_while(|&pos| in_bounds(size, pos))
        .for_each(|pos| { antinodes.insert(pos); });

    (0..)
        .map(|i| (b.0 + diff.0 * i, b.1 + diff.1 * i))
        .take_while(|&pos| in_bounds(size, pos))
        .for_each(|pos| { antinodes.insert(pos); });
}

fn count_antinodes(input: &str, add_antinodes: AddAntinodesFn) -> usize {
    let mut antennas = HashMap::new();
    
    let size = (
        input.lines().next().unwrap().len() as isize,
        input.lines().count() as isize,
    );

    input
        .lines()
        .enumerate()
        .for_each(|(j, row)| {
            row.chars().enumerate().filter(|(_, c)| c.is_alphanumeric()).for_each(|(i, c)| {
                antennas.entry(c).or_insert_with(Vec::new).push((i,j));
            })
        });

    let mut antinodes = HashSet::new();

    for (_, indexes) in antennas {
        indexes
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| ((a.0 as isize, a.1 as isize),(b.0 as isize, b.1 as isize)))
            .for_each(|(a, b)| {
                add_antinodes(a, b, size, &mut antinodes);
            });
    }

    antinodes.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(count_antinodes(input, add_antinodes))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_antinodes(input, add_resonant_antinodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
