use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_points: usize = 0;
    let mut points = 0;

    for line in input.lines() {
        let (winners, have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let winning_numbers: Vec<usize> = winners
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let numbers_you_have: Vec<usize> = have
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let matches = winning_numbers
            .iter()
            .filter(|num| numbers_you_have.iter().contains(num))
            .count();

        if matches == 0 {
            points = 0
        } else {
            points = usize::pow(2, (matches - 1) as u32)
        }

        total_points += points;
    }

    println!("{}", total_points);
    Some(total_points as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all_cards = vec![];
    for line in input.lines() {
        let (winners, have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let winning_numbers: Vec<usize> = winners
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let numbers_you_have: Vec<usize> = have
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let matches = winning_numbers
            .iter()
            .filter(|num| numbers_you_have.iter().contains(num))
            .count();

        all_cards.push(matches);
    }

    let mut counts = vec![1; all_cards.len()];
    let len = counts.len() - 1;

    for (i, one_card) in all_cards.iter().enumerate() {
        let j = if one_card > &len { len } else { one_card + i };

        for k in i + 1..j + 1 {
            counts[k] += counts[i];
        }
    }
    let res = counts.iter().sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
