use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn nodes(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut computers = HashMap::new();

    input
        .lines()
        .map(|line| line.split('-').collect::<Vec<_>>())
        .for_each(|v| {
            computers
                .entry(v[0])
                .or_insert_with(HashSet::new)
                .insert(v[1]);
            computers
                .entry(v[1])
                .or_insert_with(HashSet::new)
                .insert(v[0]);
        });

    computers
}

// https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch<'a>(
    current: &HashSet<&'a str>,
    potential: &mut HashSet<&'a str>,
    excluded: &mut HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    if potential.is_empty() && excluded.is_empty() {
        if current.len() > 2 {
            let mut clique: Vec<&str> = current.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = potential
        .union(excluded)
        .max_by_key(|node| graph.get(*node).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = graph.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<&str> = potential.difference(&neighbors).cloned().collect();

        for node in candidates {
            // New R is R ∪ {v}
            let mut next = current.clone();
            next.insert(node);

            // New P is P ∩ N(v)
            let neighbors = graph.get(&node).cloned().unwrap_or_default();
            let mut new_potential = potential
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<&str>>();

            // New X is X ∩ N(v)
            let mut new_excluded = excluded
                .intersection(&neighbors)
                .cloned()
                .collect::<HashSet<&str>>();

            // Recursive call
            bron_kerbosch(&next, &mut new_potential, &mut new_excluded, graph, cliques);

            // Move v from P to X
            potential.remove(&node);
            excluded.insert(node);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let computers = nodes(input);

    let mut groups = HashSet::new();
    for (a, links) in &computers {
        for (i, &b) in links.iter().enumerate() {
            for &c in links.iter().skip(i + 1) {
                if computers[b].contains(c) {
                    let mut set = [a, b, c];
                    if set.iter().any(|name| name.starts_with('t')) {
                        set.sort();
                        groups.insert(set);
                    }
                }
            }
        }
    }

    Some(groups.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let computers = nodes(input);

    let current = HashSet::new();
    let mut excluded = HashSet::new();
    let mut potential = computers.keys().cloned().collect::<HashSet<_>>();

    let mut cliques = Vec::new();
    bron_kerbosch(
        &current,
        &mut potential,
        &mut excluded,
        &computers,
        &mut cliques,
    );

    let clique = cliques.iter().max_by_key(|clique| clique.len()).unwrap();

    Some(clique.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
