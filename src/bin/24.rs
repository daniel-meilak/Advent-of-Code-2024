use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(24);

fn calculate(gate: String, set_bits: &mut HashMap<String, bool>, connections: &HashMap<String, Vec<String>>) -> bool {
    if let Some(found_gate) = set_bits.get(&gate) {
        return *found_gate;
    }

    let equation = &connections[&gate];
    let lhs = calculate(equation[0].to_owned(), set_bits, connections);
    let rhs = calculate(equation[2].to_owned(), set_bits, connections);

    let result = match equation[1].as_str() {
        "AND" => lhs && rhs,
        "OR" => lhs || rhs,
        "XOR" => lhs ^ rhs,
        _ => panic!(),
    };
    
    set_bits.insert(gate, result);

    result
}

fn parse(input: &str) -> (HashMap<String, bool>, HashMap<String, Vec<String>>) {
    let (initial_bits, instructions) = input.split_once("\n\n").unwrap();

    let set_bits: HashMap<_,_> = initial_bits
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(gate, value)| (gate.to_owned(), value.parse::<usize>().unwrap() != 0))
        .collect();

    let connections: HashMap<_,Vec<_>> = instructions
        .lines()
        .map(|line| line
            .split(|c| [' ', '-', '>'].contains(&c))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>())
        .map(|v| (v[3].to_owned(), vec![v[0].to_owned(), v[1].to_owned(), v[2].to_owned()]))
        .collect();

    (set_bits, connections)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut set_bits, connections) = parse(input);

    for gate in connections.keys() {
        let result = calculate(gate.clone(), &mut set_bits, &connections);
        set_bits.insert(gate.clone(), result);
    }

    let sum = set_bits
        .iter()
        .filter_map(|(gate, value)| if gate.starts_with('z') { Some((gate, *value as u64)) } else { None })
        .sorted_by(|(a, _), (b, _)|  b.cmp(a))
        .fold(0, |acc, (_, b)| (acc << 1) | (b));
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, connections) = parse(input);
    
    let max_z = connections
        .keys()
        .filter(|result| result.starts_with('z'))
        .max()
        .unwrap();

    // going by rules of ripple carry adder
    // all but final input to z?? uses XOR
    let mut swapped: HashSet<_> = connections
        .iter()
        .filter(|(result, _ )| result.starts_with('z') && *result != max_z)
        .sorted_by_key(|(gate, _)| *gate)
        .filter_map(|(result, v)| if v[1] != "XOR" { Some(result) } else { None })
        .collect();

    // all XOR must either be between x && y or result in z
    swapped.extend(connections
        .iter()
        .filter(|(_, v)| v[1] == "XOR")
        .filter(|(result, v)| !(result.starts_with('z') || v[0].starts_with('x') || v[0].starts_with('y')))
        .map(|(result, _)| result));

    // outputs of AND operations...
    let ands: HashSet<_> = connections
        .iter()
        .filter_map(|(result, v)| if v[1] == "AND" && v[0] != "x00" { Some(result) } else { None })
        .cloned()
        .collect();

    // inuputs of OR operations...
    let ors: HashSet<_> = connections
        .values()
        .filter(|v| v[1] == "OR")
        .flat_map(|v| vec![&v[0], &v[2]])
        .cloned()
        .collect();

    // shoult be a complete set (the odd ones out are mixed wires)
    swapped.extend(ands.symmetric_difference(&ors));

    let names = swapped
        .iter()
        .sorted()
        .join(",");

    Some(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("bfw,bqk,ffh,frj,fst,hwm,kpj,kwq,mjb,nrd,ntg,rvg,tgd,tnw,vdt,wpb,x00,x01,x03,y00,y02,y03,y04,z02,z03,z05,z06,z07,z08,z10,z11".to_owned()));
    }
}
