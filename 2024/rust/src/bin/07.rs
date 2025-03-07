advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut output: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        output.push((
            result.parse().unwrap(),
            numbers.split(' ').map(|s| s.parse().unwrap()).collect(),
        ));
    }

    for (left_result, numbers) in output {
        let combinations_count = 1 << (numbers.len() - 1);

        for i in 0..combinations_count {
            let mut numbers_iter = numbers.iter();
            let mut test_result = *numbers_iter.next().unwrap();

            for (j, n) in numbers_iter.enumerate() {
                if (i >> j) & 1 == 0 {
                    test_result += n; // Add if the j-th bit is 0
                } else {
                    test_result *= n; // Multiply if the j-th bit is 1
                }
            }

            // Check if the result matches left_result
            if test_result == left_result {
                result += left_result;
                break; // Exit loop if a match is found
            }
        }
    }
    // println!("left side: {:?}",left_side);
    //// Part 1: 5837374519342 (46.0ms)
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut output: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        output.push((
            result.parse().unwrap(),
            numbers.split(' ').map(|s| s.parse().unwrap()).collect(),
        ));
    }

    for (left_result, numbers) in output {
        for mut i in 0..3_u32.pow(u32::try_from(numbers.len() - 1).unwrap()) {
            let mut numbers = numbers.iter();
            let mut test_result = *numbers.next().unwrap();
            for n in numbers {
                if i % 3 == 0 {
                    test_result += n;
                } else if i % 3 == 1 {
                    test_result *= n;
                } else {
                    test_result *= 10_u64.pow(n.ilog10() + 1);
                    test_result += n;
                }
                i /= 3;
            }

            if test_result == left_result {
                result += left_result;
                break;
            }
        }
    }

    // println!("left side: {:?}",left_side);

    //// Part 1: 5837374519342 (46.0ms)
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
