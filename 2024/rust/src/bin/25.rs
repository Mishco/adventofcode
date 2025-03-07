advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let all_lines = input.split("\n\n");
    for block in all_lines {
        let x = block.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
        if x[0][0] == b'#' {
            locks.push(x);
        } else {
            keys.push(x);
        }
    }
    let mut result = 0;

    for l in &locks {
        for k in &keys {
            let mut is_ok = true;
            for r in 0..l.len() {
                for c in 0..l[0].len() {
                    if l[r][c] == b'#' && k[r][c] == b'#' {
                        is_ok = false;
                    }
                }
            }
            if is_ok {
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
