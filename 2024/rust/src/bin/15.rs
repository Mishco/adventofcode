use itertools::Itertools;

advent_of_code::solution!(15);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_movements(input: &str) -> Vec<Direction> {
    let movements: Vec<Direction> = input
        .lines()
        .filter(|line| !line.starts_with('#'))
        .flat_map(|s| {
            s.chars().filter_map(|c| match c {
                '^' => Some(Direction::Up),
                '>' => Some(Direction::Right),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                _ => None,
            })
        })
        .collect();
    movements
}

fn get_current_move(m: &Direction) -> (isize, isize) {
    let (i, j): (isize, isize) = match m {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };
    (i, j)
}

fn find_robot_position(matrix: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row_index, row) in matrix.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == '@') {
            return Some((row_index, col_index));
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .filter_map(|line| {
            if line.trim() != "" && line.trim().starts_with('#') {
                Some(line.trim().chars().collect())
            } else {
                None
            }
        })
        .collect();

    let movements = parse_movements(input);
    let (mut robot_i, mut robot_j) = find_robot_position(&matrix).unwrap();

    for m in movements {
        let (i, j) = get_current_move(&m);

        let npi = (robot_i as isize + i) as usize;
        let npj = (robot_j as isize + j) as usize;
        if matrix[npi][npj] == '.' {
            matrix[robot_i][robot_j] = '.';
            robot_i = npi;
            robot_j = npj;
            matrix[npi][npj] = '@';
        } else if matrix[npi][npj] == '#' {
            continue;
        } else if matrix[npi][npj] == 'O' {
            let mut iterations = 1;
            loop {
                let inpi = (robot_i as isize + i * iterations) as usize;
                let inpj = (robot_j as isize + j * iterations) as usize;
                if matrix[inpi][inpj] == '#' {
                    break;
                } else if matrix[inpi][inpj] == '.' {
                    matrix[robot_i][robot_j] = '.';
                    matrix[npi][npj] = '@';
                    matrix[inpi][inpj] = 'O';
                    robot_i = npi;
                    robot_j = npj;
                    break;
                } else if matrix[inpi][inpj] == 'O' {
                } else {
                    panic!("Unexpected char");
                }
                iterations += 1;
            }
        } else {
            panic!("Unexpected char");
        }
    }
    let result = (0..matrix.len())
        .cartesian_product(0..matrix[0].len())
        .filter(|&(r, c)| matches!(matrix[r][c], 'O' | '['))
        .map(|(r, c)| r * 100 + c)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .filter_map(|line| {
            if line.trim() != "" && line.trim().starts_with('#') {
                let mut row: Vec<char> = vec![];
                line.trim().chars().for_each(|c| match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '.' => {
                        row.push('.');
                        row.push('.');
                    }
                    '@' => {
                        row.push('@');
                        row.push('.');
                    }
                    _ => panic!("Unexpected char"),
                });
                Some(row)
            } else {
                None
            }
        })
        .collect();

    let movements = parse_movements(input);
    let (mut robot_i, mut robot_j) = find_robot_position(&matrix).unwrap();

    for m in movements {
        let (i, j) = get_current_move(&m);

        let npi = (robot_i as isize + i) as usize;
        let npj = (robot_j as isize + j) as usize;
        if matrix[npi][npj] == '.' {
            matrix[robot_i][robot_j] = '.';
            robot_i = npi;
            robot_j = npj;
            matrix[npi][npj] = '@';
        } else if matrix[npi][npj] == '#' {
            continue;
        } else if matrix[npi][npj] == '[' || matrix[npi][npj] == ']' {
            let mut iterations = 1;

            match m {
                Direction::Left | Direction::Right => loop {
                    let inpi = (robot_i as isize + i * iterations) as usize;
                    let inpj = (robot_j as isize + j * iterations) as usize;
                    if matrix[inpi][inpj] == '#' {
                        break;
                    } else if matrix[inpi][inpj] == '.' {
                        matrix[robot_i][robot_j] = '.';
                        matrix[inpi][inpj] = matrix[npi][npj];
                        // flip every parenthesis
                        for z in 1..iterations + 1 {
                            let zi = (robot_i as isize + i * z) as usize;
                            let zj = (robot_j as isize + j * z) as usize;
                            if matrix[zi][zj] == '[' {
                                matrix[zi][zj] = ']';
                            } else if matrix[zi][zj] == ']' {
                                matrix[zi][zj] = '[';
                            } else {
                                panic!("Unexpected char");
                            }
                        }

                        matrix[npi][npj] = '@';
                        robot_i = npi;
                        robot_j = npj;
                        break;
                    } else if matrix[inpi][inpj] == '[' || matrix[inpi][inpj] == ']' {
                    } else {
                        panic!("Unexpected char");
                    }
                    iterations += 1;
                },
                Direction::Up | Direction::Down => {
                    let mut connected: Vec<(usize, usize)> = vec![];
                    let mut stack: Vec<(usize, usize)> = vec![];
                    connected.push((npi, npj));
                    stack.push((npi, npj));
                    let mut can_move = true;
                    while let Some((current_i, current_j)) = stack.pop() {
                        let i1 = (current_i as isize + i) as usize;
                        let j1 = (current_j as isize + j) as usize;
                        let j2 = match matrix[current_i][current_j] {
                            '[' => (current_j as isize + j) as usize + 1,
                            ']' => (current_j as isize + j) as usize - 1,
                            _ => panic!("Unexpected char"),
                        };

                        if matrix[i1][j1] == '#' || matrix[i1][j2] == '#' {
                            can_move = false;
                            break;
                        }
                        if matrix[i1][j1] == '[' || matrix[i1][j1] == ']' {
                            stack.push((i1, j1));
                            connected.push((i1, j1));
                        }
                        if (matrix[current_i][current_j] == '[' && matrix[i1][j2] == '[')
                            || (matrix[current_i][current_j] == ']' && matrix[i1][j2] == ']')
                        {
                            stack.push((i1, j2));
                            connected.push((i1, j2));
                        }
                    }

                    if can_move {
                        while let Some((current_i, current_j)) = connected.pop() {
                            if matrix[current_i][current_j] != '['
                                && matrix[current_i][current_j] != ']'
                            {
                                continue;
                            }
                            let i1 = (current_i as isize + i) as usize;
                            let j1 = (current_j as isize + j) as usize;
                            let j2 = match matrix[current_i][current_j] {
                                '[' => (current_j as isize + j) as usize + 1,
                                ']' => (current_j as isize + j) as usize - 1,
                                _ => {
                                    panic!("Unexpected char for {} {}", current_i, current_j)
                                }
                            };

                            if matrix[i1][j1] != '.' || matrix[i1][j2] != '.' {
                                continue;
                            }

                            let current_j2 = match matrix[current_i][current_j] {
                                '[' => current_j + 1,
                                ']' => current_j - 1,
                                _ => panic!("Unexpected char"),
                            };

                            matrix[i1][j1] = matrix[current_i][current_j];
                            matrix[i1][j2] = matrix[current_i][current_j2];
                            matrix[current_i][current_j] = '.';
                            matrix[current_i][current_j2] = '.';
                        }

                        matrix[robot_i][robot_j] = '.';
                        matrix[npi][npj] = '@';
                        robot_i = npi;
                        robot_j = npj;
                    }
                }
            }
        } else {
            panic!("Unexpected char");
        }
    }
    let result = (0..matrix.len())
        .cartesian_product(0..matrix[0].len())
        .filter(|&(r, c)| matches!(matrix[r][c], 'O' | '['))
        .map(|(r, c)| r * 100 + c)
        .sum();

    Some(result)
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
