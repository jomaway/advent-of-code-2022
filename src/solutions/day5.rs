use std::{num::ParseIntError, path::Path, str::FromStr};

use itertools::Itertools;

use super::get_input_by_block;

pub fn run() {
    let path = Path::new("src/inputs/day5.txt");

    println!("Running tasks of day 5");

    let res1 = task1(path);
    println!("Result of task1 is {}", res1);

    let res2 = task2(path);
    println!("Result of task2 is {}", res2);
}

fn create_stacks(input: &str) -> [Vec<char>; 9] {
    let mut cargo: [Vec<char>; 9] = Default::default();

    for group in input.lines().map(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|g| g[1])
            .collect::<Vec<char>>()
    }) {
        for (idx, val) in group.iter().enumerate() {
            match val {
                ' ' => (),
                _ => cargo[idx].insert(0, *val),
            }
        }
    }

    cargo
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Vec<&str> = s.split_whitespace().collect();

        let instr = Instruction {
            amount: steps[1].parse::<usize>()?,
            from: steps[3].parse::<usize>()? - 1,
            to: steps[5].parse::<usize>()? - 1,
        };

        Ok(instr)
    }
}

pub fn task1(path: &Path) -> String {
    let input = get_input_by_block(path);
    let mut stacks = create_stacks(&input[0]);
    let instructions = input[1]
        .lines()
        .map(|i| i.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for i in instructions {
        for _ in 0..i.amount {
            if let Some(char) = stacks[i.from].pop() {
                stacks[i.to].push(char);
            }
        }
    }

    let mut result = String::new();
    for mut stack in stacks {
        if let Some(char) = stack.pop() {
            result.push(char);
        }
    }
    return result;
}

pub fn task2(path: &Path) -> String {
    let input = get_input_by_block(path);
    let mut stacks = create_stacks(&input[0]);
    let instructions = input[1]
        .lines()
        .map(|i| i.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for i in instructions {
        let mut crane = stacks[i.from].split_off(stacks[i.from].len() - i.amount);
        stacks[i.to].extend(crane);
    }

    let mut result = String::new();
    for mut stack in stacks {
        if let Some(char) = stack.pop() {
            result.push(char);
        }
    }
    return result;
}

mod tests {
    use super::{task1, task2};
    use std::path::Path;

    const PATH: &str = "src/inputs/day5-e.txt";

    #[test]
    fn test_task1() {
        let path = Path::new(PATH);
        let expected_result: &str = "CMZ";

        assert_eq!(task1(path), expected_result)
    }

    #[test]
    #[ignore]
    fn test_task2() {
        let path = Path::new(PATH);
        let expected_result: &str = "MCD";

        assert_eq!(task2(path), expected_result)
    }
}
