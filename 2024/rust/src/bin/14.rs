use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(14);

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

type Position = (i32, i32);
type Velocity = (i32, i32);
type RobotType = (Position, Velocity);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect_vec();
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots = lines
        .iter()
        .map(|line| {
            let caps_iter = regex.captures(line).unwrap();
            let mut caps = caps_iter
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>().unwrap());
            Robot {
                position: (caps.next().unwrap(), caps.next().unwrap()),
                velocity: (caps.next().unwrap(), caps.next().unwrap()),
            }
        })
        .collect_vec();

    let mut quadrants: HashMap<(bool, bool), usize> = HashMap::new();
    const ITERS: i32 = 100;
    for Robot {
        position: (x, y),
        velocity: (dx, dy),
    } in robots.iter()
    {
        let fx = (x + (dx * ITERS)).rem_euclid(WIDTH);
        let fy = (y + (dy * ITERS)).rem_euclid(HEIGHT);
        if fx == WIDTH / 2 || fy == HEIGHT / 2 {
            continue;
        }
        *quadrants
            .entry((fx < WIDTH / 2, fy < HEIGHT / 2))
            .or_default() += 1;
    }

    let total = quadrants.values().product::<usize>();
    Some(total)
}
fn simulate_robots(robots: &[RobotType], t: i32) -> Vec<(i32, i32)> {
    let max_x = 101;
    let max_y = 103;

    robots
        .iter()
        .map(|&((rx, ry), (vx, vy))| {
            (
                (rx + t * vx).rem_euclid(max_x),
                (ry + t * vy).rem_euclid(max_y),
            )
        })
        .collect()
}

fn count_in_formation(robots: &HashSet<(i32, i32)>) -> usize {
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut touching = 0;
    for &(rx, ry) in robots {
        for (dx, dy) in deltas.iter() {
            if robots.contains(&(rx + dx, ry + dy)) {
                touching += 1;
                break;
            }
        }
    }
    touching
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"-?\d+").unwrap();

    let mut robots = Vec::new();
    for line in input.lines() {
        let p: Vec<i32> = re
            .captures_iter(line)
            .map(|cap| cap[0].parse().unwrap())
            .collect();
        robots.push(((p[0], p[1]), (p[2], p[3])));
    }

    let mut t = 1;
    loop {
        let positions: HashSet<(i32, i32)> = simulate_robots(&robots, t).into_iter().collect();
        if count_in_formation(&positions) >= positions.len() / 2 {
            break;
        }
        t += 1;
    }
    Some(t as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // WIDTH = 11;
        // HEIGHT = 7;
        // const WIDTH: i32 = 11;
        // const HEIGHT: i32 = 7; should be 12

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        println!("{:?}", result); //
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
