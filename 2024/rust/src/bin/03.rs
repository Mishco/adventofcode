advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // let pattern = r"mul\(\d\(+\),\s*\d\(+\)\)"; // Corrected pattern
    let pattern = r"mul\((\d+),\s*(\d+)\)"; // Matches mul(...) with one or more digits
    let regex = Regex::new(pattern).unwrap(); // Create the regex object

    // let mut matches = Vec::new();

    // for mat in regex.find_iter(input) {
    //     matches.push(mat.as_str());
    // }

    // println!("{:?}", matches);
    let mut total = 0;
    for mat in regex.captures_iter(input) {
        let first = mat[1].parse::<u32>().unwrap();
        let second = mat[2].parse::<u32>().unwrap();
        let mul_result = first * second;

        total += mul_result;
    }
    // let mul = input.find("mul(\d{3},{3})").iter().collect();
    // println!("{:?}", mul);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut is_enabled = true;

    let result = pattern
        .captures_iter(input)
        .filter_map(|captures| {
            if let (Some(x), Some(y)) = (captures.get(1), captures.get(2)) {
                if is_enabled {
                    let x = x.as_str().parse::<u32>().unwrap();
                    let y = y.as_str().parse::<u32>().unwrap();
                    // println!("{} ({}, {})", is_enabled, x, y);
                    Some(x * y)
                } else {
                    None
                }
            } else {
                match &captures[0] {
                    "don't()" => is_enabled = false,
                    "do()" => is_enabled = true,
                    _ => {}
                }
                None
            }
        })
        .sum::<u32>();

    Some(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
