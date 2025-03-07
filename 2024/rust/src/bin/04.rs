advent_of_code::solution!(4);

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
];

fn count_xmas_occurrences(grid: Vec<Vec<char>>, word: &str) -> usize {
    let mut result: u32 = 0;
    let word_length: usize = word.chars().count();

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == word.chars().next().unwrap() {
                for &d in &DIRECTIONS {
                    let mut found = true;

                    for j in 1..word_length {
                        let nx = x as i32 + d.0 * j as i32;
                        let ny = y as i32 + d.1 * j as i32;

                        if nx < 0
                            || ny < 0
                            || nx >= grid[0].len() as i32
                            || ny >= grid.len() as i32
                            || grid[ny as usize][nx as usize] != word.chars().nth(j).unwrap()
                        {
                            found = false; // If any character does not match, break
                            break;
                        }
                    }

                    if found {
                        result += 1;
                    }
                }
            }
        }
    }

    result as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let chars = input.lines().map(|line| line.chars().collect()).collect();
    let result = count_xmas_occurrences(chars, "XMAS");
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' && i > 0 && j > 0 && i < grid[0].len() - 1 && j < grid.len() - 1 {
                let c1 = grid[i - 1][j - 1];
                let c2 = grid[i + 1][j + 1];
                let c3 = grid[i - 1][j + 1];
                let c4 = grid[i + 1][j - 1];
                if ((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M'))
                    && ((c3 == 'M' && c4 == 'S') || (c3 == 'S' && c4 == 'M'))
                {
                    result += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
