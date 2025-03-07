use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules = input
        .lines()
        .take_while(|line| line.len() > 1)
        .map(|line| {
            line.split("|")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|pages| (pages[0], pages[1]))
        .collect::<std::collections::HashSet<_>>();
    let result: usize = input
        .lines()
        .skip(rules.len() + 1)
        .map(|update| {
            update
                .split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|update| update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = input
        .lines()
        .take_while(|line| line.len() > 1)
        .map(|line| {
            line.split("|")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|pages| (pages[0], pages[1]))
        .collect::<std::collections::HashSet<_>>();
    let result: usize = input
        .lines()
        .skip(rules.len() + 1)
        .map(|update| {
            update
                .split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|update| !update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
