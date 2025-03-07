use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};

advent_of_code::solution!(16);

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Pos(i32, i32);
//
// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct ReindeerArena {
//     legal: Vec<Pos>,
//     illegal: Vec<Pos>,
//     end: Pos,
//     start: Pos,
// }

// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Vector {
//     pos: Pos,
//     dir: Pos,
// }
//
// impl ReindeerArena {
//     fn is_legal(&self, pos: &Pos) -> bool {
//         self.legal.contains(pos) && !self.illegal.contains(pos)
//     }
// }

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    x: i32,
    y: i32,
    di: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Position = (i32, i32);

fn rotate_clockwise((dx, dy): Position) -> Position {
    (-dy, dx)
}

fn rotate_counterclockwise((dx, dy): Position) -> Position {
    (dy, -dx)
}

fn move_forward((x, y): Position, (dx, dy): Position) -> Position {
    (x + dx, y + dy)
}
pub fn part_one(input: &str) -> Option<usize> {
    // read to map/grid
    let lines = input.lines().collect_vec();
    let mut start = (0, 0); // Point(12, 1);
    let mut end = (0, 0); // Point(1, 12);
    let mut walls: HashSet<Position> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                WALL => {
                    walls.insert(pos);
                }
                START => {
                    start = pos;
                }
                END => {
                    end = pos;
                }
                _ => {}
            }
        }
    }

    let successors = |(p, facing): &(Position, Position)| {
        let pos = *p;

        // // backtracking path with
        // // one move forward = 1 point
        // // one rotate (90degrees) = 1000points
        let movements = [
            (*facing, 1),
            (rotate_clockwise(*facing), 1001),
            (rotate_counterclockwise(*facing), 1001),
        ];

        movements
            .iter()
            .map(move |(facing, cost)| {
                let new_pos = move_forward(pos, *facing);
                ((new_pos, *facing), *cost)
            })
            .filter(|((next_pos, _), _)| !walls.contains(next_pos))
            .collect::<Vec<_>>()
    };
    let success = |(pos, _): &(Position, Position)| pos == &end;
    // djiskstra to find a path
    let (_, min_cost) = dijkstra(&(start, (1, 0)), successors, success).unwrap();

    Some(min_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect_vec();
    let mut start = (0, 0); // Point(12, 1);
    let mut end = (0, 0); // Point(1, 12);
    let mut walls: HashSet<Position> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                WALL => {
                    walls.insert(pos);
                }
                START => {
                    start = pos;
                }
                END => {
                    end = pos;
                }
                _ => {}
            }
        }
    }

    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let start_state = State {
        score: 0,
        x: start.0,
        y: start.1,
        di: 0, // start facing east
    };
    heap.push(Reverse(start_state));

    let mut seen = vec![usize::MAX; width * height * DIRS.len()];
    let mut min = usize::MAX;

    while let Some(Reverse(State {
        score,
        x,
        y,
        di: prev_di,
    })) = heap.pop()
    {
        if grid[y as usize * width + x as usize] == b'E' {
            if score > min {
                // This path is worse than any best path we have found before.
                break;
            }
            min = score;
        }

        for (di, (dx, dy)) in DIRS.iter().enumerate() {
            if (prev_di + 2) % DIRS.len() == di {
                // don't go back
                continue;
            }

            let nscore = if di == prev_di {
                // move forwards
                score + 1
            } else {
                // turn and just one step
                score + 1001
            };

            let nx = x + dx;
            let ny = y + dy;

            let gi = ny as usize * width + nx as usize;
            let si = gi * DIRS.len() + di;
            let last_seen_score = seen[si];

            if grid[gi] != b'#' && nscore <= last_seen_score {
                seen[si] = nscore;

                heap.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    di,
                }));
            }
        }
    }

    let mut total = 1;
    let mut places_to_sit = vec![false; width * height];
    let mut queue = VecDeque::new();
    queue.push_back((end, min));
    while let Some((node, score)) = queue.pop_front() {
        for di in 0..DIRS.len() {
            let next_score = seen[(node.1 as usize * width + node.0 as usize) * DIRS.len() + di];
            if next_score <= score {
                // walk back
                let nextx = node.0 - DIRS[di].0;
                let nexty = node.1 - DIRS[di].1;

                if !places_to_sit[nexty as usize * width + nextx as usize] {
                    places_to_sit[nexty as usize * width + nextx as usize] = true;
                    total += 1;
                    queue.push_back(((nextx, nexty), next_score));
                }
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
