fn part1(input: &str) -> i32 {
    let mut previous_value = -1;
    let mut counter = 0;
    for line in input.lines() {
        if previous_value == -1 {
            println!("{line} (N/A - no previous measurement)");
            previous_value = line.parse::<i32>().unwrap();
        } else if line.parse::<i32>().unwrap() < previous_value {
            println!("{line} (decreased)");
            previous_value = line.parse::<i32>().unwrap();
        } else {
            println!("{line} (increase)");
            previous_value = line.parse::<i32>().unwrap();
            counter += 1
        }
    }
    counter
}

fn part2(input: &str) -> i32 {
    let mut previous_value = None;
    let mut counter = 0;
    let slice_size = 3;
    let overlap = 2;
    let step_size = slice_size - overlap;

    let lines: Vec<&str> = input.lines().collect();

    for i in (0..=lines.len() - slice_size).step_by(step_size) {
        let line_slice: Vec<i32> = lines[i..i + slice_size]
            .iter()
            .map(|line| line.trim().parse::<i32>().unwrap_or(0))
            .collect();

        let sum_line: i32 = line_slice.iter().sum();

        match previous_value {
            None => {
                println!("{sum_line} (N/A - no previous measurement)");
            }
            Some(prev) => {
                match sum_line.cmp(&prev) {
                    std::cmp::Ordering::Less => println!("{} (decreased)", sum_line),
                    std::cmp::Ordering::Equal => println!("{} (no change)", sum_line),
                    std::cmp::Ordering::Greater => {
                        println!("{} (increased)", sum_line);
                        counter += 1; // Increment the counter for increases
                    }
                }
            }
        }
        previous_value = Some(sum_line);
    }

    counter
}

fn main() {
    let test_input = "199
200
208
210
200
207
240
269
260
263
";

    let res = part1(test_input);
    assert_eq!(res, 7);

    let input = include_str!("../../inputs/2021-01.txt");
    let res = part1(input);
    println!("{res}");

    let res = part2(test_input);
    assert_eq!(res, 5);
    let res = part2(input);
    println!("{res}");
}
