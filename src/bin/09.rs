advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut memory = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let times = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            memory.resize(memory.len() + times, Some(i / 2));
        } else {
            memory.resize(memory.len() + times, None);
        }
    }

    let mut checksum = 0;

    let mut i = 0;
    while i < memory.len() {
        if memory[i].is_none() {
            while memory.last().unwrap().is_none() {
                memory.pop();
            }

            if memory.len() <= i {
                break;
            }

            memory[i] = memory.pop().unwrap();
        }

        checksum += i * memory[i].unwrap();
        i += 1;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut memory = Vec::new();
    let mut position = 0;
    for c in input.chars() {
        let length = c.to_digit(10).unwrap() as usize;

        memory.push((position, length));
        position += length;
    }

    let (mut filled_memory, mut free_memory): (Vec<_>, Vec<_>) = memory
        .into_iter()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    for (_, filled) in filled_memory.iter_mut().rev() {
        for (_, free) in free_memory.iter_mut() {
            if free.0 <= filled.0 && free.1 >= filled.1 {
                // update the position of the moved memory
                filled.0 = free.0;

                // update the position of the free space
                free.0 += filled.1;

                // reduce size of free space (taken up now by filled memory)
                free.1 -= filled.1;
            }
        }
    }

    let checksum: usize = filled_memory
        .iter()
        .enumerate()
        .map(|(pos, (_, block))| pos * (2 * block.0 + block.1 - 1) * block.1 / 2)
        .sum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
