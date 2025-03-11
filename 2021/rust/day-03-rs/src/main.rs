use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let most_common_bit: i32;
    let gamma_rate=0;
    let epsilon_rate = 0;

    let binary_numbers: Vec<&str> = input.lines().collect();
    let num_columns = binary_numbers[0].len();

    let num_bits = binary_numbers[0].len();
    assert!(binary_numbers.iter().all(|x| x.len()==num_bits));

    for

    gamma_rate * epsilon_rate
}

fn part2(input: &str) -> i32 {
    todo()!
}

fn main() {
    let test_int = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    let res = part1(test_int);
    assert_eq!(res, 198);
    println!("{}", res);

}
