use std::iter::zip;
use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {

    let (seeds_part, distances_part) = input.split_once("\r\n")?;
    let ttt: Vec<i32> = seeds_part.split_once(":")?
        .1.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    
    let ddd: Vec<i32> = distances_part.split_once(":")?
        .1.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    // let ttt = val.0.split_once(":").unwrap().1.split_whitespace().map(i32::from_str).collect::<Vec<_>>();
    // let ddd = val.1.split_once(":").unwrap().1.split_whitespace().map(i32::from_str).collect::<Vec<_>>();

    println!("{:?} {:?}", ttt, ddd);

    let mut ret = 1;
    for (time, dist) in zip(ttt, ddd)  {
        let mut wins = 0;
        for i in (0..time) {
            if i * (time - i) > dist {
                wins +=1
            }
        }
        ret *= wins
    }

    Some(ret as u32)
    
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
