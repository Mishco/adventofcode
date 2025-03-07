advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    // let mut unsafe_count = 0;
    // input.lines().map(|line| {
    //     let numbers = line.split_whitespace().map(|item| {
    //         item.parse::<u32>().unwrap()
    //     }).collect();
    //     if checking_level(numbers) {
    //         safe_count += 1
    //     }
    // }).sum();

    for line in input.lines() {
        // println!("{}", line);

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();

        if checking_level(&numbers) {
            safe_count += 1;
        }
    }

    // println!("{}",safe_count);
    Some(safe_count as u32)
}

fn checking_level(numbers: &[i32]) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..numbers.len() - 1 {
        // let diff = (&numbers - &numbers).abs();
        let curr = numbers[i];
        let next = numbers[i + 1];

        let diff = (next - curr).abs();

        if !(1..=3).contains(&diff) {
            //unsafe
            return false;
        }

        match curr < next {
            true => is_decreasing = false,
            false => is_increasing = false,
        }
    }
    is_increasing || is_decreasing
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    // let mut unsafe_count = 0;
    for line in input.lines() {
        // println!("{}", line);

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        if checking_level(&numbers) {
            safe_count += 1;
        } else {
            // try to remove item and check again
            for i in 0..numbers.len() {
                let mut cleaned_level = numbers.to_vec();
                cleaned_level.remove(i);

                if checking_level(&cleaned_level) {
                    safe_count += 1;
                    break;
                }
            }
            // unsafe_count += 1;
        }
    }

    // println!("{}",safe_count);
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
