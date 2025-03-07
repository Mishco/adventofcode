use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(21);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

#[cached]
fn complexity(from: char, to: char, mut robot_count: i32) -> u64 {
    robot_count -= 1;

    let dir_layout: HashMap<char, Pos> = [
        ('^', Pos(1, 0)),
        ('A', Pos(2, 0)),
        ('<', Pos(0, 1)),
        ('v', Pos(1, 1)),
        ('>', Pos(2, 1)),
    ]
    .iter()
    .cloned()
    .collect();

    let num_layout: HashMap<char, Pos> = [
        ('7', Pos(0, 0)),
        ('8', Pos(1, 0)),
        ('9', Pos(2, 0)),
        ('4', Pos(0, 1)),
        ('5', Pos(1, 1)),
        ('6', Pos(2, 1)),
        ('1', Pos(0, 2)),
        ('2', Pos(1, 2)),
        ('3', Pos(2, 2)),
        ('0', Pos(1, 3)),
        ('A', Pos(2, 3)),
    ]
    .iter()
    .cloned()
    .collect();

    let layout = if dir_layout.contains_key(&from) && dir_layout.contains_key(&to) {
        &dir_layout
    } else {
        &num_layout
    };

    let start = layout[&from];
    let end = layout[&to];

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    // Directions to move
    let dx_dir = if dx > 0 { ">" } else { "<" };
    let dy_dir = if dy > 0 { "v" } else { "^" };

    // Create the steps vector
    let steps: Vec<&str> = vec![dx_dir; dx.unsigned_abs() as usize]
        .into_iter()
        .chain(vec![dy_dir; dy.unsigned_abs() as usize])
        .collect();

    let mut paths = vec![];
    let mut stack = vec![(Vec::new(), steps)];

    // Paths
    while let Some((current_path, remaining_steps)) = stack.pop() {
        if remaining_steps.is_empty() {
            let mut complete_path = current_path;
            complete_path.push("A");
            paths.push(complete_path.join(""));
        } else {
            for (i, &step) in remaining_steps.iter().enumerate() {
                let mut next_path = current_path.clone();
                next_path.push(step);

                let mut next_steps = remaining_steps.clone();
                next_steps.remove(i);

                stack.push((next_path, next_steps));
            }
        }
    }

    paths.retain(|path| is_valid_path(path, start, layout));

    if dx == 0 && dy == 0 {
        paths.push("A".to_string());
    }

    if robot_count > 0 {
        return paths
            .iter()
            .map(|path| {
                let path_with_a = format!("A{}", path);
                path_with_a
                    .chars()
                    .tuple_windows()
                    .map(|(a, b)| complexity(a, b, robot_count))
                    .sum::<u64>()
            })
            .min()
            .expect("Problem finding min");
    }

    paths
        .iter()
        .map(|path| path.len() as u64)
        .min()
        .expect("Problem finding min")
}

fn is_valid_path(path: &str, start: Pos, layout: &HashMap<char, Pos>) -> bool {
    let mut current_pos = start;
    for c in path.chars() {
        current_pos = match c {
            '^' => Pos(current_pos.0, current_pos.1 - 1),
            'v' => Pos(current_pos.0, current_pos.1 + 1),
            '<' => Pos(current_pos.0 - 1, current_pos.1),
            '>' => Pos(current_pos.0 + 1, current_pos.1),
            _ => current_pos,
        };

        if !layout.values().any(|&pos| pos == current_pos) {
            return false;
        }
    }
    true
}

fn calculate_complexity_sum(input: &str, robot_count: i32) -> u64 {
    input
        .lines()
        .map(|line| {
            let num: u64 = line.chars().take(3).collect::<String>().parse().unwrap();
            let seq = format!("A{}", line);
            let mut complexity_sum = 0;

            for (a, b) in seq.chars().tuple_windows() {
                complexity_sum += complexity(a, b, robot_count);
            }

            complexity_sum * num
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let total_complexity = calculate_complexity_sum(input, 3);
    Some(total_complexity as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let total_complexity = calculate_complexity_sum(input, 26);
    Some(total_complexity as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
