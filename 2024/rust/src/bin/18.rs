use std::collections::VecDeque;

advent_of_code::solution!(18);

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Free,
    Byte,
}
struct Grid {
    tiles: Vec<Vec<Space>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Space::Free; width]; height];
        Self {
            tiles,
            height,
            width,
        }
    }

    fn corrupted_space(&mut self, row: usize, col: usize) {
        self.tiles[row][col] = Space::Byte;
    }

    fn corrupt_space(&mut self, row: usize, col: usize) {
        self.tiles[row][col] = Space::Byte;
    }

    fn corrupt_spaces(&mut self, spaces: &[(usize, usize)]) {
        for &(row, col) in spaces {
            self.corrupt_space(row, col);
        }
    }
    fn exists_in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.height as isize && col >= 0 && col < self.width as isize
    }

    fn is_free(&self, row: usize, col: usize) -> bool {
        self.tiles[row][col] == Space::Free
    }

    fn shortest_path(&self) -> usize {
        self.find_path(true)
    }

    fn path_exists(&self) -> bool {
        self.find_path(false) != usize::MAX
    }

    fn find_path(&self, return_distance: bool) -> usize {
        let start = (0, 0);
        let end = (self.height - 1, self.width - 1);
        let mut to_visit: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];

        to_visit.push_back((start.0, start.1, 0));

        while let Some((row, col, dist)) = to_visit.pop_front() {
            if (row, col) == end {
                return if return_distance { dist } else { 1 }; // return 1 if path exists
            }

            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;

            for (dr, dc) in DIRECTIONS {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if self.exists_in_bounds(new_row, new_col)
                    && self.is_free(new_row as usize, new_col as usize)
                {
                    let new_dist = dist + 1;
                    to_visit.push_back((new_row as usize, new_col as usize, new_dist));
                }
            }
        }
        usize::MAX
    }

    fn clear_bytes(&mut self) {
        for row in &mut self.tiles {
            row.fill(Space::Free);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_called::<71, 1024>(input)
}

pub fn part_one_called<const RANGE: usize, const TAKEN_N: usize>(input: &str) -> Option<usize> {
    let mut grid = Grid::new(RANGE, RANGE);
    for byte in input.lines().take(TAKEN_N) {
        let (row, col) = byte.split_once(',').unwrap();

        let row = row.parse().unwrap();
        let col = col.parse().unwrap();

        grid.corrupted_space(row, col);
    }
    Some(grid.shortest_path())
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_called::<71, 1024>(input)
}

pub fn part_two_called<const RANGE: usize, const TAKEN_N: usize>(input: &str) -> Option<String> {
    let mut grid = Grid::new(RANGE, RANGE);

    let bytes = input
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(usize, usize)>>();

    let mut max = bytes.len();
    let mut min = RANGE + 1;

    while min != max {
        let mid = (min + max) / 2;

        grid.corrupt_spaces(&bytes[..=mid]);
        if grid.path_exists() {
            min = mid + 1;
        } else {
            max = mid;
        }

        grid.clear_bytes();
    }
    let (first_part, second_part) = bytes[max];
    let result = format!("{:?},{:?}", first_part, second_part);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_called::<7, 12>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two_called::<7, 12>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
