use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut freqs = HashMap::<_, Vec<_>>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' {
                continue;
            }
            freqs.entry(grid[i][j]).or_default().push((i, j));
        }
    }

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                positions.entry(*c).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes_part1 = HashSet::new();
    let mut antinodes_part2 = HashSet::new();
    for antennas in positions.values() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let dx = antennas[j].0 - antennas[i].0;
                let dy = antennas[j].1 - antennas[i].1;
                let mut n = 1;
                loop {
                    let nxi = antennas[i].0 + dx * n;
                    let nyi = antennas[i].1 + dy * n;
                    let nxj = antennas[j].0 - dx * n;
                    let nyj = antennas[j].1 - dy * n;

                    let mut ok = 0;

                    if nxi >= 0 && nyi >= 0 && nxi < grid[0].len() as i32 && nyi < grid.len() as i32
                    {
                        if n == 2 {
                            antinodes_part1.insert((nxi, nyi));
                        }
                        antinodes_part2.insert((nxi, nyi));
                        ok += 1;
                    }

                    if nxj >= 0 && nyj >= 0 && nxj < grid[0].len() as i32 && nyj < grid.len() as i32
                    {
                        if n == 2 {
                            antinodes_part1.insert((nxj, nyj));
                        }
                        antinodes_part2.insert((nxj, nyj));
                        ok += 1;
                    }

                    if ok == 0 {
                        break;
                    }

                    n += 1;
                }
            }
        }
    }

    Some(antinodes_part1.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                positions.entry(*c).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes_part1 = HashSet::new();
    let mut antinodes_part2 = HashSet::new();
    for antennas in positions.values() {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let dx = antennas[j].0 - antennas[i].0;
                let dy = antennas[j].1 - antennas[i].1;
                let mut n = 1;
                loop {
                    let nxi = antennas[i].0 + dx * n;
                    let nyi = antennas[i].1 + dy * n;
                    let nxj = antennas[j].0 - dx * n;
                    let nyj = antennas[j].1 - dy * n;

                    let mut ok = 0;

                    if nxi >= 0 && nyi >= 0 && nxi < grid[0].len() as i32 && nyi < grid.len() as i32
                    {
                        if n == 2 {
                            antinodes_part1.insert((nxi, nyi));
                        }
                        antinodes_part2.insert((nxi, nyi));
                        ok += 1;
                    }

                    if nxj >= 0 && nyj >= 0 && nxj < grid[0].len() as i32 && nyj < grid.len() as i32
                    {
                        if n == 2 {
                            antinodes_part1.insert((nxj, nyj));
                        }
                        antinodes_part2.insert((nxj, nyj));
                        ok += 1;
                    }

                    if ok == 0 {
                        break;
                    }

                    n += 1;
                }
            }
        }
    }

    // println!("{}", antinodes_part2.len());
    Some(antinodes_part2.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
