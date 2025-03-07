use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    /// load input and parse
    // let mut all_games =  HashMap::new();
    let mut games: HashMap<String, String> = HashMap::new();

    let ll = input.lines();
    for line in ll {
        let split = line.split(":").collect::<Vec<&str>>();
        let game_id = split[0];
        let ls_ = split[1];
        if !game_id.is_empty() {
            games.insert(String::from(game_id), ls_.parse().unwrap());
        }
    }

    let mut red_values: Vec<i32> = Vec::new();
    let mut blue_values: Vec<i32> = Vec::new();
    let mut green_values: Vec<i32> = Vec::new();

    let mut new_games: HashMap<String, Vec<HashMap<String, Vec<i32>>>> = HashMap::new();

    for (game_id, game) in games.iter() {
        for item in game.split(';') {
            for value in item.split(',') {
                let parts: Vec<&str> = value.split_whitespace().collect();

                if parts.len() < 2 {
                    continue; // Skip if the format is unexpected
                }

                let num: i32 = parts[0].trim().parse().unwrap_or(0); // Safely parse the number

                if value.contains("red") {
                    red_values.push(num);
                } else if value.contains("blue") {
                    blue_values.push(num);
                } else if value.contains("green") {
                    green_values.push(num);
                }
            }
        }

        new_games.insert(
            game_id.clone(),
            vec![
                HashMap::from([("blue".to_string(), blue_values.clone())]),
                HashMap::from([("green".to_string(), green_values.clone())]),
                HashMap::from([("red".to_string(), red_values.clone())]),
            ],
        );

        // Reset values for the next game
        red_values.clear();
        blue_values.clear();
        green_values.clear();
    }

    // Display all collected values
    println!("Red values: {:?}", red_values);
    println!("Blue values: {:?}", blue_values);
    println!("Green values: {:?}", green_values);

    // Setting minimal resources
    let minimal_green = 13;
    let minimal_red = 12;
    let minimal_blue = 14;
    let mut sum = 0;
    let mut total_power = 0;

    for (key, val) in new_games.iter() {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for invalue in val {
            if let Some(blue) = invalue.get("blue") {
                max_blue = blue.iter().cloned().max().unwrap_or(0);
            } else if let Some(green) = invalue.get("green") {
                max_green = green.iter().cloned().max().unwrap_or(0);
            } else if let Some(red) = invalue.get("red") {
                max_red = red.iter().cloned().max().unwrap_or(0);
            }
        }

        println!(
            "{:?}: Max red: {}, Max green: {}, Max blue: {}",
            key, max_red, max_green, max_blue
        );

        let power = max_red * max_green * max_blue;
        total_power += power;

        if minimal_red >= max_red && minimal_green >= max_green && minimal_blue >= max_blue {
            println!("{} would be possible", key);

            let just_number: i32 = key
                .split_whitespace()
                .last()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0);

            sum += just_number;
        }
    }
    println!("\nSum: {}", sum);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut games: HashMap<String, String> = HashMap::new();

    let ll = input.lines();
    for line in ll {
        let split = line.split(":").collect::<Vec<&str>>();
        let game_id = split[0];
        let ls_ = split[1];
        if !game_id.is_empty() {
            games.insert(String::from(game_id), ls_.parse().unwrap());
        }
    }

    let mut red_values: Vec<i32> = Vec::new();
    let mut blue_values: Vec<i32> = Vec::new();
    let mut green_values: Vec<i32> = Vec::new();

    let mut new_games: HashMap<String, Vec<HashMap<String, Vec<i32>>>> = HashMap::new();

    for (game_id, game) in games.iter() {
        for item in game.split(';') {
            for value in item.split(',') {
                let parts: Vec<&str> = value.split_whitespace().collect();

                if parts.len() < 2 {
                    continue; // Skip if the format is unexpected
                }

                let num: i32 = parts[0].trim().parse().unwrap_or(0); // Safely parse the number

                if value.contains("red") {
                    red_values.push(num);
                } else if value.contains("blue") {
                    blue_values.push(num);
                } else if value.contains("green") {
                    green_values.push(num);
                }
            }
        }

        new_games.insert(
            game_id.clone(),
            vec![
                HashMap::from([("blue".to_string(), blue_values.clone())]),
                HashMap::from([("green".to_string(), green_values.clone())]),
                HashMap::from([("red".to_string(), red_values.clone())]),
            ],
        );

        // Reset values for the next game
        red_values.clear();
        blue_values.clear();
        green_values.clear();
    }

    // Display all collected values
    println!("Red values: {:?}", red_values);
    println!("Blue values: {:?}", blue_values);
    println!("Green values: {:?}", green_values);

    // Setting minimal resources
    let minimal_green = 13;
    let minimal_red = 12;
    let minimal_blue = 14;
    let mut sum = 0;
    let mut total_power = 0;

    for (key, val) in new_games.iter() {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for invalue in val {
            if let Some(blue) = invalue.get("blue") {
                max_blue = blue.iter().cloned().max().unwrap_or(0);
            } else if let Some(green) = invalue.get("green") {
                max_green = green.iter().cloned().max().unwrap_or(0);
            } else if let Some(red) = invalue.get("red") {
                max_red = red.iter().cloned().max().unwrap_or(0);
            }
        }

        println!(
            "{:?}: Max red: {}, Max green: {}, Max blue: {}",
            key, max_red, max_green, max_blue
        );

        let power = max_red * max_green * max_blue;
        total_power += power;

        if minimal_red >= max_red && minimal_green >= max_green && minimal_blue >= max_blue {
            println!("{} would be possible", key);

            let just_number: i32 = key
                .split_whitespace()
                .last()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0);

            sum += just_number;
        }
    }
    println!("\nSum: {}", sum);

    Some(total_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
