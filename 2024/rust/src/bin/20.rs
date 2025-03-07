advent_of_code::solution!(20);

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn find_start_end(g: Vec<&[u8]>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            match g[r][c] {
                b'S' => start = (r, c),
                b'E' => end = (r, c),
                _ => {}
            }
        }
    }
    (start, end)
}

fn compute_distances(
    g: Vec<&[u8]>,
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut q = VecDeque::from([(start.0, start.1, 0usize)]);
    let mut dists = HashMap::new();

    while let Some((r, c, n)) = q.pop_front() {
        if dists.contains_key(&(r, c)) {
            continue;
        }
        dists.insert((r, c), n);
        if (r, c) == end {
            continue;
        }
        for (dr, dc) in DIRECTIONS {
            let (rr, cc) = (r.wrapping_add(dr as usize), c.wrapping_add(dc as usize));
            if g.get(rr)
                .and_then(|row| row.get(cc))
                .is_some_and(|&cell| cell != b'#')
            {
                q.push_back((rr, cc, n + 1));
            }
        }
    }

    dists
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_called::<100, 2>(input)
}

pub fn part_one_called<const MIN_TIME: usize, const MAX_DISTANCE: usize>(
    input: &str,
) -> Option<usize> {
    let g: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (start, end) = find_start_end(g.clone());
    let dists = compute_distances(g.clone(), start, end);

    let mut p1 = 0;
    for ((&(r1, c1), &n1), (&(r2, c2), &n2)) in dists.iter().tuple_combinations() {
        let d = r1.abs_diff(r2) + c1.abs_diff(c2);
        if d <= 20 && n2.abs_diff(n1) >= d + MIN_TIME && d <= MAX_DISTANCE {
            p1 += 1;
        }
    }
    Some(p1)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_called::<100, 20>(input)
}

pub fn part_two_called<const MIN_TIME: usize, const MAX_DISTANCE: usize>(
    input: &str,
) -> Option<u32> {
    let g: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (start, end) = find_start_end(g.clone());
    let dists = compute_distances(g, start, end);

    let mut p2 = 0;
    for ((&(r1, c1), &n1), (&(r2, c2), &n2)) in dists.iter().tuple_combinations() {
        let d = r1.abs_diff(r2) + c1.abs_diff(c2);
        if d <= MAX_DISTANCE && n2.abs_diff(n1) >= d + MIN_TIME {
            p2 += 1;
        }
    }
    Some(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_called::<2, 2>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two_called::<50, 20>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
