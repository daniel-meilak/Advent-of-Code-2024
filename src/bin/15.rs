advent_of_code::solution!(15);

fn add(lhs: usize, rhs: i32) -> usize {
    (lhs as i32 + rhs) as usize
}

fn shift(grid: &mut [Vec<char>], positions: &mut [(usize, usize)], dir: (i32, i32)) {
    positions.sort();

    let range: Box<dyn Iterator<Item = &(usize, usize)>> = match dir {
        (-1, 0) | (0, -1) => Box::new(positions.iter()),
        (1, 0) | (0, 1) => Box::new(positions.iter().rev()),
        _ => panic!(),
    };

    let mut new_positions = Vec::new();

    for pos in range {
        let next = (add(pos.0, dir.0), add(pos.1, dir.1));
        grid[next.1][next.0] = grid[pos.1][pos.0];
        new_positions.push(next);
    }

    for pos in positions {
        if !new_positions.contains(pos) {
            grid[pos.1][pos.0] = '.';
        }
    }
}

fn sum_gps(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter().enumerate().filter_map(move |(i, &c)| {
                if "O[".contains(c) {
                    Some(i + j * 100)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn find_robot(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .flat_map(|(j, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(i, &c)| if c == '@' { Some((i, j)) } else { None })
        })
        .next()
        .unwrap()
}

fn get_dir(direction: char) -> (i32, i32) {
    match direction {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!(),
    }
}

fn move_robot_small_boxes(mut robot: (usize, usize), grid: &mut [Vec<char>], movements: &str) {
    for direction in movements.chars() {
        if direction == '\n' {
            continue;
        }
        let dir = get_dir(direction);

        let mut stack = vec![robot];
        let mut next = (add(robot.0, dir.0), add(robot.1, dir.1));
        loop {
            match grid[next.1][next.0] {
                '.' => {
                    shift(grid, &mut stack, dir);
                    robot = (add(robot.0, dir.0), add(robot.1, dir.1));
                    break;
                }
                'O' => stack.push(next),
                '#' => break,
                _ => panic!(),
            }

            next = (add(next.0, dir.0), add(next.1, dir.1));
        }
    }
}

fn move_robot_large_boxes(mut robot: (usize, usize), grid: &mut [Vec<char>], movements: &str) {
    for direction in movements.chars() {
        if direction == '\n' {
            continue;
        }
        let dir = get_dir(direction);

        let mut stack = vec![robot];

        loop {
            let mut can_move = 0;
            let mut blocked = false;

            for part in &stack {
                let next = (add(part.0, dir.0), add(part.1, dir.1));

                match grid[next.1][next.0] {
                    '.' => can_move += 1,
                    x @ '[' | x @ ']' => {
                        if stack.contains(&next) {
                            can_move += 1;
                            continue;
                        } else {
                            stack.push(next);
                            stack.push((add(next.0, if x == '[' { 1 } else { -1 }), next.1));
                            break;
                        }
                    }
                    '#' => blocked = true,
                    _ => panic!(),
                }
            }

            if blocked {
                break;
            }

            if can_move == stack.len() {
                shift(grid, &mut stack, dir);
                robot = (add(robot.0, dir.0), add(robot.1, dir.1));
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid_input, movements) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = grid_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let robot = find_robot(&grid);

    move_robot_small_boxes(robot, &mut grid, movements);

    Some(sum_gps(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid_input, movements) = input.split_once("\n\n").unwrap();
    let mut wide_grid = String::new();

    for c in grid_input.chars() {
        match c {
            '#' => wide_grid.push_str("##"),
            'O' => wide_grid.push_str("[]"),
            '.' => wide_grid.push_str(".."),
            '@' => wide_grid.push_str("@."),
            '\n' => wide_grid.push('\n'),
            _ => panic!(),
        }
    }

    let mut grid: Vec<Vec<char>> = wide_grid
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let robot = find_robot(&grid);

    move_robot_large_boxes(robot, &mut grid, movements);

    Some(sum_gps(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
