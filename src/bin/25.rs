advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let locks_and_keys: Vec<&str> = input.split("\n\n").collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    
    for item in locks_and_keys {
        if item.starts_with("#####") {
            locks.push(item);
        } else {
            keys.push(item);
        }
    }

    let mut overlapping = 0;
    for lock in locks {
        for key in &keys {
            overlapping += lock.chars()
                .zip(key.chars())
                .all(|(a, b)| !(a == '#' && b == '#')) as u32;
            println!()
        }
    }
    
    Some(overlapping)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
