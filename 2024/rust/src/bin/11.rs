advent_of_code::solution!(11);
use rustc_hash::FxHashMap;
fn solve(input: &str, steps: usize) -> usize {
    let mut stones: FxHashMap<u64, usize> = input
        .trim()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok().map(|num| (num, 1)))
        .collect();
    for _ in 0..steps {
        // let stones_cloned = stones.clone();
        for (stone, n) in stones.drain().collect::<Vec<_>>() {
            let mut insert = |s| {
                stones.entry(s).and_modify(|x| *x += n).or_insert(n);
            };
            if stone == 0 {
                insert(1);
            } else {
                match (stone as f32).log10().floor() as u32 + 1 {
                    digits if digits % 2 == 0 => {
                        insert(stone / 10u64.pow(digits / 2));
                        insert(stone % 10u64.pow(digits / 2));
                    }
                    _ => insert(stone * 2024),
                }
            }
        }
    }
    stones.values().sum()
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
