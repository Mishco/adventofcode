advent_of_code::solution!(23);
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    // Parse the input into a graph keeping track of vertices and edges.
    let (vertices, edges) = parse_input(input);

    // For part 1, simply find all the cliques of size 3 and count the ones that
    // contain a "t" in one of the vertices.
    let cliques = find_cliques_n(&vertices, &edges, 3);

    let res = cliques
        .iter()
        .filter(|clique| clique.iter().any(|v| v.starts_with('t')))
        .count();

    Some(res)
}

fn parse_input(input: &str) -> (HashSet<&str>, HashMap<&str, HashSet<&str>>) {
    input.lines().fold(
        (HashSet::new(), HashMap::new()),
        |(mut vertices, mut edges), line| {
            let (left, right) = line.split_once("-").unwrap();
            vertices.insert(left);
            vertices.insert(right);
            edges.entry(left).or_insert_with(HashSet::new).insert(right);
            edges.entry(right).or_insert_with(HashSet::new).insert(left);
            (vertices, edges)
        },
    )
}

fn find_cliques_n<'a>(
    vertices: &HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
    n: usize,
) -> HashSet<Vec<&'a str>> {
    let mut cliques = HashSet::new();

    // We basically test every vertex for a clique of size k. To do this, we
    // start by adding all of the vertices to a queue with a clique of just
    // itself.
    let mut queue = vertices
        .iter()
        .map(|&v| (v, vec![v]))
        .collect::<VecDeque<_>>();

    while let Some((vertex, mut clique)) = queue.pop_front() {
        // If we have found a clique of size k, we add it to the set of cliques.
        if clique.len() == n {
            clique.sort();
            cliques.insert(clique.clone());
            continue;
        }

        // Get a list of neighbors for the current vertex.
        let mut neighbors = edges.get(vertex).unwrap().clone();
        neighbors.retain(|neighbor| {
            clique
                .iter()
                .all(|v| edges.get(v).unwrap().contains(neighbor))
        });

        // Add all of the valid neighbors to the queue with the current clique.
        for neighbor in neighbors {
            let mut new_clique = clique.clone();
            new_clique.push(neighbor);
            queue.push_back((neighbor, new_clique));
        }
    }

    // Return all the cliques we found.
    cliques
}

pub fn part_two(input: &str) -> Option<String> {
    let (vertices, edges) = parse_input(input);
    let largest_clique = bron_kerbosch(
        &HashSet::new(),
        &mut vertices.iter().cloned().collect(),
        &mut HashSet::new(),
        &edges,
    );

    let mut largest_clique = largest_clique.iter().collect::<Vec<_>>();
    largest_clique.sort();

    let res = largest_clique.iter().join(",");
    Some(res)
}

/// Bron-Kerbosch algorithm is a recursive algorithm for finding all maximal cliques in an undirected graph.
fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    // If we have checked all our potential vertices and have no excluded
    // vertices, we have found a clique. We check if the current clique is
    // larger than the largest one we have found so far and store it.
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    // Try adding each vertex in p to the current clique.
    let mut largest_clique = HashSet::new();
    for vertex in p.clone() {
        // Make a new r with the current vertex added.
        let mut r = r.clone();
        r.insert(vertex);

        // Make a new p and x where their vertices are the intersection of the
        // current vertex's neighbors and the current p and x.
        let neighbors = edges.get(vertex).unwrap();
        let mut new_p = p.intersection(neighbors).cloned().collect();
        let mut new_x = x.intersection(neighbors).cloned().collect();

        // Recurse with the new r, p, and x.
        let clique = bron_kerbosch(&r, &mut new_p, &mut new_x, edges);
        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }

        // Move the current vertex from p to x for the next iteration.
        p.remove(vertex);
        x.insert(vertex);
    }

    largest_clique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
