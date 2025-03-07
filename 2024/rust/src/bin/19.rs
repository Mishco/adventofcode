advent_of_code::solution!(19);

struct Node {
    next: [usize; 6],
}

impl Node {
    fn new() -> Self {
        Node { next: [0; 6] }
    }

    fn set_towel(&mut self) {
        self.next[3] = 1;
    }

    fn towels(&self) -> usize {
        self.next[3]
    }
}

fn is_available_to_construct(design: &str, patterns: &Vec<&str>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if let Some(remaining_design) = design.strip_prefix(pattern) {
                if is_available_to_construct(remaining_design, patterns) {
                    return true;
                }
            }
        }
    }

    false
}

fn count_possible_designs(available_patterns: Vec<&str>, desired_designs: Vec<&str>) -> usize {
    let mut possible_count = 0;
    for design in desired_designs {
        if is_available_to_construct(design, &available_patterns) {
            possible_count += 1;
        }
    }
    possible_count
}

pub fn part_one(input: &str) -> Option<usize> {
    // let available_patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    // let desired_designs = vec![
    //     "brwrr",
    //     "bggr",
    //     "gbbr",
    //     "rrbgbr",
    //     "ubwu",
    //     "bwurrg",
    //     "brgr",
    //     "bbrgwb",
    // ];

    let (available_patterns, desired_designs) =
        input.split_once("\n\n").expect("Invalid input format");
    let available_patterns = available_patterns
        .trim()
        .split(',')
        .map(|s| s.trim())
        .collect();
    let desired_designs = desired_designs.lines().map(|s| s.trim()).collect();

    let possible_designs_count = count_possible_designs(available_patterns, desired_designs);
    Some(possible_designs_count)
}

fn perfect_hash(b: u8) -> usize {
    let n = b as usize;
    (n ^ (n >> 4)) % 8
}

pub fn part_two(input: &str) -> Option<usize> {
    let (available_patterns, desired_designs) = input.split_once("\n\n").unwrap();

    // Build Trie
    let mut trie = Vec::with_capacity(1000);
    trie.push(Node::new());

    for towel in available_patterns.split(", ") {
        let mut i = 0;

        for j in towel.bytes().map(perfect_hash) {
            if trie[i].next[j] == 0 {
                trie[i].next[j] = trie.len();
                i = trie.len();
                trie.push(Node::new());
            } else {
                i = trie[i].next[j];
            }
        }
        trie[i].set_towel();
    }
    let mut part_two = 0;
    let mut ways = Vec::with_capacity(100);

    for design in desired_designs.lines().map(str::as_bytes) {
        let size = design.len();

        ways.clear();
        ways.resize(size + 1, 0);

        ways[0] = 1;

        for start in 0..size {
            if ways[start] > 0 {
                let mut i = 0;

                for end in start..size {
                    i = trie[i].next[perfect_hash(design[end])];

                    if i == 0 {
                        break;
                    }

                    ways[end + 1] += trie[i].towels() * ways[start];
                }
            }
        }
        let total = ways[size];
        part_two += total;
    }
    Some(part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
