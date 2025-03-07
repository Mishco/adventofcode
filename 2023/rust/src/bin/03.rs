advent_of_code::solution!(3);
use grid::Grid;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Part {
    location: Point,
    part_type: char,
}

fn create_grid(input: &str) -> Grid<char> {
    // Create a grid and fill it with our input
    let mut grid: Grid<char> = Grid::new(0, 0);
    for l in input.lines() {
        grid.push_row(l.chars().collect_vec());
    }
    grid
}

fn parse_part(location: (usize, usize), part_type: &char) -> Option<Part> {
    match part_type {
        // Filter out any dots or numbers, keeping all the parts
        '0'..='9' | '.' => None,
        _ => Some(Part {
            part_type: *part_type,
            location: Point {
                x: location.0,
                y: location.1,
            },
        }),
    }
}
fn find_adjacent_points(point: &Point) -> Vec<Point> {
    let mut adjacent_points: Vec<Point> = vec![];

    // Loop around the point generating a vec
    // There are no parts on the edge of the schematic so we do not worry about over/underflowing
    for x in (point.x - 1)..=point.x + 1 {
        for y in (point.y - 1)..=(point.y + 1) {
            adjacent_points.push(Point { x, y });
        }
    }

    adjacent_points
}

fn discover_numbers(part: &Part, grid: &Grid<char>) -> Vec<u32> {
    let adjacent_points = find_adjacent_points(&part.location);

    let re = Regex::new(r"\d+").unwrap();
    let mut matches: Vec<u32> = vec![];

    for x in (part.location.x - 1)..=part.location.x + 1 {
        let row = grid.iter_row(x).collect::<String>();

        for m in re.find_iter(&row) {
            let match_range = m.start()..m.end();

            for y in match_range {
                if adjacent_points.contains(&Point { x, y }) {
                    //Parse the match and push the result into the part
                    matches.push(m.as_str().parse::<u32>().unwrap());
                    // Move onto the next regex match if a gear is touching
                    break;
                }
            }
        }
    }
    matches
}

fn get_parts_list(grid: Grid<char>) -> Vec<(Part, Vec<u32>)> {
    grid.indexed_iter()
        .filter_map(|(location, part_type)| parse_part(location, part_type))
        .map(|part| {
            // Discover the matches for the part
            let matches = discover_numbers(&part, &grid);
            (part, matches)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = create_grid(input);
    let parts: Vec<(Part, Vec<u32>)> = get_parts_list(grid);

    Some(
        parts
            .iter()
            .map(|(_, matches)| matches.iter().sum::<u32>())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = create_grid(input);
    let parts: Vec<(Part, Vec<u32>)> = get_parts_list(grid);

    Some(
        parts
            .iter()
            .filter_map(|(part, matches)| match part.part_type {
                '*' => {
                    if matches.len() == 2 {
                        Some(matches.iter().product::<u32>())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
