advent_of_code::solution!(17);
fn load_program(lines: Vec<&str>) -> Vec<u64> {
    lines[4][9..]
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn call_computer_output(program: &[u64], mut a: u64, mut b: u64, mut c: u64) -> Vec<u64> {
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip + 1];

        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match opcode {
            0 => a >>= combo,
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => unreachable!(), //println!("wrong {:?}", opcode),
        };

        ip += 2;
    }
    output
}

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<_>>();

    //read register
    let a = lines[0][12..].parse::<u64>().ok()?;
    let b = lines[1][12..].parse::<u64>().ok()?;
    let c = lines[2][12..].parse::<u64>().ok()?;

    let program = load_program(lines);

    // println!("{:?} {:?}",a, program);
    let output = call_computer_output(&program, a, b, c);

    Some(
        output
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    // we know what suppose to be register A
    let lines = input.lines().collect::<Vec<_>>();

    // read register
    let _a = lines[0][12..].parse::<u64>().ok()?;
    let b = lines[1][12..].parse::<u64>().ok()?;
    let c = lines[2][12..].parse::<u64>().ok()?;

    // println!("a: {} b: {} c: {}", a, b, c);

    let program = load_program(lines);

    let mut possibilities = vec![0; program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in possibilities.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f
        }

        let output = call_computer_output(&program, init_a, b, c);

        if output == program {
            return Some(init_a as usize);
        }

        for i in (0..program.len()).rev() {
            if output.len() < i || output[i] != program[i] {
                possibilities[i] += 1;
                for f in possibilities.iter_mut().take(i) {
                    *f = 0;
                }
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
