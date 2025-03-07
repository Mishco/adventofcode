advent_of_code::solution!(10);

use std::collections::HashMap;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

pub fn part_one_another(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    // Collect coordinates of the trailheads (cells with value 0)
    let trailheads: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_id, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                if col == 0 {
                    Some((row_id, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut total_count = 0;

    for (x, y) in trailheads {
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((x, y));
        let mut existing: HashMap<(usize, usize), bool> = HashMap::new();
        while let Some((cur_x, cur_y)) = stack.pop() {
            let cur_val = matrix[cur_x][cur_y];
            if cur_val == 9 {
                existing.insert((cur_x, cur_y), true);
                continue;
            }
            for (dx, dy) in DIRECTIONS.iter() {
                let new_x = cur_x as isize + dx;
                let new_y = cur_y as isize + dy;

                // Check if the new coordinates are within bounds
                if new_x >= 0
                    && new_x < matrix.len() as isize
                    && new_y >= 0
                    && new_y < matrix[0].len() as isize
                {
                    // Verify if the value in the matrix is equal to cur_val + 1
                    if matrix[new_x as usize][new_y as usize] == cur_val + 1 {
                        stack.push((new_x as usize, new_y as usize));
                    }
                }
            }
        }
        total_count += existing.len() as u32;
        existing.clear();
    }

    Some(total_count)
}

pub fn part_one(input: &str) -> Option<usize> {
    let result: usize = part_one_another(input)? as usize;

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: usize = 0;

    let matrix: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let trailheads: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                if col == 0 {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    for (x, y) in trailheads {
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((x, y));

        while let Some((current_x, current_y)) = stack.pop() {
            let value = matrix[current_x][current_y];
            if value == 9 {
                result += 1;
                continue;
            }

            for (dx, dy) in DIRECTIONS.iter() {
                let new_x: usize = (current_x as isize + dx) as usize;
                let new_y: usize = (current_y as isize + dy) as usize;

                if new_x < matrix.len()
                    && new_y < matrix[0].len()
                    && matrix[new_x][new_y] == value + 1
                {
                    stack.push((new_x, new_y));
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
