use itertools::Itertools;
use regex::Regex;
use rust_utils::utils::modulus;

advent_of_code::solution!(14);

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse(input: &str) -> Vec<Robot> {
    let regex = Regex::new("p=(.*),(.*) v=(.*),(.*)").unwrap();

    let matches = regex
        .captures_iter(input)
        .flat_map(|captures| {
            vec![
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ]
        })
        .collect_vec();

    matches
        .chunks(4)
        .map(|c| Robot { x: c[0], y: c[1], vx: c[2], vy: c[3] })
        .collect_vec()
}

fn move_robots(robots: &mut [Robot], width: i32, height: i32) {
    robots
    .iter_mut()
    .for_each(|robot| {
        robot.x = modulus(robot.x + robot.vx, width);
        robot.y = modulus(robot.y + robot.vy, height);
    });
}

fn average_distance(robots: &[Robot]) -> f64 {
    let mut distance = 0.0;
    for i in 0..robots.len() {
        for j in 0..robots.len() {
            if i == j { continue; }

            let mut d =  (robots[i].x - robots[j].x).pow(2) as f64 + (robots[i].y - robots[j].y).pow(2) as f64;
            d = d.sqrt();

            distance += d;
        }
    }

    distance / robots.len() as f64
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse(input);

    let (width, height) = if cfg!(test) {
        (11, 7)
    } else {
        (101, 103)
    };
    
    for _ in 0..100 {
        move_robots(&mut robots, width, height)
    }

    let mut quadrants = (0, 0, 0, 0);
    for robot in robots {
        if robot.x < width/2  && robot.y < height/2 {
            quadrants.0 += 1;
        } else if robot.x < width/2 && robot.y > height/2 {
            quadrants.1 += 1;
        } else if robot.x  > width/2 && robot.y < height/2 {
            quadrants.2 += 1;
        } else if robot.x > width/2 && robot.y > height/2 {
            quadrants.3 += 1;
        }
    }

    Some(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots  = parse(input);

    let (width, height) = if cfg!(test) {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut distances = Vec::new();
    for _ in 0..100 {
        move_robots(&mut robots, width, height);
        distances.push(average_distance(&robots));
    }
    
    let average = distances.iter().sum::<f64>()/distances.len() as f64;

    let variance = distances
        .iter()
        .map(|distance| (distance - average).powi(2))
        .sum::<f64>() / distances.len() as f64;

    let standard_deviation = variance.sqrt();

    let mut seconds = 100;
    loop {
        move_robots(&mut robots, width, height);
        seconds += 1;
        if (average_distance(&robots) - average).abs() > 8.0 * standard_deviation {
            break;
        }
    }

    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
