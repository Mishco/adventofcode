advent_of_code::solution!(24);

use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    logic: Logic,
    a: &'a str,
    b: &'a str,
    out: &'a str,
}

fn compute_bit_vector(s: &HashMap<&str, bool>, p: char) -> usize {
    s.keys()
        .filter(|&n| n.starts_with(p))
        .map(|n| {
            let index = n[1..].parse::<usize>().unwrap();
            if s[n] {
                1 << index
            } else {
                0
            }
        })
        .fold(0, |acc, value| acc | value) // Combine the bit values
}

fn val<'a>(
    gates: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    states: &mut HashMap<&'a str, bool>,
    node: &'a str,
) -> bool {
    if let Some(&v) = states.get(node) {
        return v;
    }
    let (a, op, b) = gates[node];
    let a = val(gates, states, a);
    let b = val(gates, states, b);

    let res = match op {
        "AND" => a && b,
        "XOR" => a != b,
        "OR" => a || b,
        _ => unreachable!(),
    };

    states.insert(node, res);
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut s = HashMap::new();
    let mut g = HashMap::new();
    for l in wires.lines() {
        let (n, v) = l.split_once(": ").unwrap();
        s.insert(n, v == "1");
    }
    for l in gates.lines() {
        let (a, op, b, _, c) = l.split(' ').collect_tuple().unwrap();
        g.insert(c, (a, op, b));
    }
    for n in g.keys() {
        val(&g, &mut s, n);
    }
    Some(compute_bit_vector(&s, 'z'))
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = input.split_once("\n\n").unwrap();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut broken_nodes = HashSet::new();

    let gates = gates
        .lines()
        .map(|l| {
            let (left, out) = l.split_once(" -> ").unwrap();
            let s = left.split_whitespace().collect::<Vec<_>>();
            let logic = match s[1] {
                "AND" => Logic::And,
                "OR" => Logic::Or,
                "XOR" => Logic::Xor,
                _ => panic!("Invalid gate: {}", left),
            };
            Gate {
                logic,
                a: s[0],
                b: s[2],
                out,
            }
        })
        .collect::<Vec<_>>();

    for g in &gates {
        edges.entry(g.a).or_default().push(g.out);
        edges.entry(g.b).or_default().push(g.out);
    }

    for g in &gates {
        // z nodes must be XOR (except for the last one, z45)
        if g.out.starts_with("z") && g.out != "z45" && g.logic != Logic::Xor {
            broken_nodes.insert(g.out);
        }
        // z nodes must not be inputs of other nodes
        if g.a.starts_with("z") {
            broken_nodes.insert(g.a);
        }
        if g.b.starts_with("z") {
            broken_nodes.insert(g.b);
        }

        // inputs of XOR nodes (except for z nodes) must be x and y nodes
        if g.logic == Logic::Xor
            && !g.out.starts_with("z")
            && !((g.a.starts_with("x") && g.b.starts_with("y"))
                || (g.a.starts_with("y") && g.b.starts_with("x")))
        {
            broken_nodes.insert(g.out);
        }

        // XOR nodes (except z nodes) must always be input of exactly two
        // other nodes
        if g.logic == Logic::Xor && !g.out.starts_with("z") && edges[g.out].len() != 2 {
            broken_nodes.insert(g.out);
        }

        // AND nodes must always be input of exactly one other node (except
        // the very first one wired to x00 and y00)
        if g.logic == Logic::And
            && !g.out.starts_with("z")
            && edges[g.out].len() != 1
            && !((g.a == "x00" && g.b == "y00") || (g.a == "y00" && g.b == "x00"))
        {
            broken_nodes.insert(g.out);
        }
    }
    let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
    broken_nodes.sort();

    Some(broken_nodes.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_with_diff_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("z00,z02".to_owned()));
    }
}
