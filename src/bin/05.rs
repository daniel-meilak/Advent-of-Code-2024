use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(5);

fn get_rules_and_updates(input: &str) -> (HashMap<&str, HashSet<&str>>, &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut before_rules = HashMap::new();

    for line in rules.lines() {
        let (before, after) = line.split('|').collect_tuple().unwrap();

        before_rules.entry(before).or_insert_with(HashSet::new).insert(after);
        before_rules.entry(after).or_default();
    }

    (before_rules, updates)
}

fn ordered(pages: &[&str], rules: &HashMap<&str, HashSet<&str>>) -> bool {
    for i in 0..pages.len() {
        for j in i+1..pages.len() {
            if !rules[pages[i]].contains(pages[j]) {
                return false;
            }
        }
    } 

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);

    let mut total = 0;
    for line in updates.lines() {
        let pages: Vec<&str> = line.split(',').collect();

        if ordered(&pages, &rules) {
            total += pages[pages.len()/2].parse::<u32>().unwrap();
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = get_rules_and_updates(input);

    let mut total = 0;
    for line in updates.lines() {
        let pages: Vec<&str> = line.split(',').collect();

        if !ordered(&pages, &rules) {
            let mut deque: VecDeque<&str> = pages.iter().copied().collect();
            let mut new_order = Vec::new();

            while new_order.len() < pages.len() {
                let page = deque.pop_front().unwrap();

                if deque.iter().all(|&p| rules[page].contains(p)) {
                    new_order.push(page);
                } else {
                    deque.push_back(page);
                }
            }

            total += new_order[new_order.len()/2].parse::<u32>().unwrap();
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
