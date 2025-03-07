advent_of_code::solution!(6);

use itertools::Itertools;
use std::collections::HashSet;

type Mtx = Vec<Vec<char>>;

fn get(mtx: &Mtx, r: i32, c: i32, r_max: i32, c_max: i32) -> Option<char> {
    if r < 0 || r >= r_max || c < 0 || c >= c_max {
        None
    } else {
        Some(mtx[r as usize][c as usize])
    }
}

fn turn_right((dr, dx): (i32, i32)) -> (i32, i32) {
    match (dr, dx) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

fn part1(mtx: &Mtx) -> Option<usize> {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let (mut r, mut c) = (0..r_max)
        .cartesian_product(0..c_max)
        .find(|&(r, c)| get(mtx, r, c, r_max, c_max) == Some('^'))?;
    let (mut dr, mut dc) = (-1, 0);
    let mut positions = HashSet::new();
    loop {
        positions.insert((r, c));
        while let Some('#') = get(mtx, r + dr, c + dc, r_max, c_max) {
            (dr, dc) = turn_right((dr, dc));
        }
        if get(mtx, r + dr, c + dc, r_max, c_max).is_none() {
            break;
        }
        (r, c) = (r + dr, c + dc);
    }
    Some(positions.len())
}

fn obstacle_loops(mtx: &mut Mtx, r: i32, c: i32, dr: i32, dc: i32, r_max: i32, c_max: i32) -> bool {
    if get(mtx, r, c, r_max, c_max) != Some('.') {
        return false;
    }
    let mut loop_found = false;
    mtx[r as usize][c as usize] = '#';
    {
        let (mut r, mut c) = (r - dr, c - dc);
        let (mut dr, mut dc) = (dr, dc);
        let mut positions = HashSet::new();
        loop {
            if !positions.insert((r, c, dr, dc)) {
                loop_found = true;
                break;
            }
            while let Some('#') = get(mtx, r + dr, c + dc, r_max, c_max) {
                (dr, dc) = turn_right((dr, dc));
            }
            if get(mtx, r + dr, c + dc, r_max, c_max).is_none() {
                break;
            }
            // if let None = get(mtx, r + dr, c + dc, r_max, c_max) {
            //     break;
            // }
            (r, c) = (r + dr, c + dc);
        }
    }
    mtx[r as usize][c as usize] = '.';
    loop_found
}

fn part2(mtx: &mut Mtx) -> Option<usize> {
    let r_max = mtx.len() as i32;
    let c_max = mtx[0].len() as i32;
    let (mut r, mut c) = (0..r_max)
        .cartesian_product(0..c_max)
        .find(|&(r, c)| get(mtx, r, c, r_max, c_max) == Some('^'))?;
    let (mut dr, mut dc) = (-1, 0);
    let mut visited = HashSet::new();
    let mut obstacles = HashSet::new();
    loop {
        visited.insert((r, c));
        while let Some('#') = get(mtx, r + dr, c + dc, r_max, c_max) {
            (dr, dc) = turn_right((dr, dc));
        }
        if get(mtx, r + dr, c + dc, r_max, c_max).is_none() {
            break;
        }
        (r, c) = (r + dr, c + dc);
        // Do not put obstacles on visited positions, yields false positives.
        if visited.contains(&(r, c)) {
            continue;
        }
        if obstacles.contains(&(r, c)) {
            continue;
        }
        // Test an obstacle.
        if obstacle_loops(mtx, r, c, dr, dc, r_max, c_max) {
            obstacles.insert((r, c));
        }
    }
    Some(obstacles.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Mtx>();
    part1(&input)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Mtx>();
    part2(&mut input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
