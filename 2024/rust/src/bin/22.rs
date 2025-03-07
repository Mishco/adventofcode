use itertools::{iterate, Itertools};

advent_of_code::solution!(22);

const MASK: i64 = (1 << 24) - 1;
const NUM_POSSIBLE_VALUES: usize = 19_usize.pow(4);
const OFFSET: i64 = 9;

#[inline]
fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) & MASK;
    secret ^= secret >> 5;
    (secret ^ secret << 11) & MASK
}

pub fn part_one(input: &str) -> Option<i64> {
    let result: i64 = input
        .lines()
        .map(|p| {
            iterate(p.parse::<i64>().expect("Invalid number format"), step)
                .nth(2000)
                .unwrap()
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut dp = vec![0; NUM_POSSIBLE_VALUES];
    let mut dpi = vec![0; NUM_POSSIBLE_VALUES];

    for (p, i) in input.lines().zip(1..) {
        let number = p.parse::<i64>().unwrap();

        for (e, d, c, b, a) in iterate(number, step)
            .take(2001)
            .map(|n| n % 10)
            .tuple_windows()
        {
            let idx = calculate_index(d - e, c - d, b - c, a - b);

            if dpi[idx] != i {
                dpi[idx] = i;
                dp[idx] += a;
            }
        }
    }
    Some(dp.into_iter().max().unwrap())
}

/// Range of -9..=9 possible for each, 19.pow(4) possible values
/// `delta_a`, `delta_b`, `delta_c`, `delta_d` represent the differences from the offset.
fn calculate_index(delta_a: i64, delta_b: i64, delta_c: i64, delta_d: i64) -> usize {
    let adjusted_a = delta_a + OFFSET;
    let adjusted_b = delta_b + OFFSET;
    let adjusted_c = delta_c + OFFSET;
    let adjusted_d = delta_d + OFFSET;

    (6859 * adjusted_a + 361 * adjusted_b + 19 * adjusted_c + adjusted_d) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
