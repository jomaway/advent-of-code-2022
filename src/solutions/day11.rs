use std::{path::Path, collections::VecDeque, fmt::Display, str::FromStr};

use itertools::Itertools;

use super::get_input_by_block;

const DAY : u8 = 11; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug,)]
struct Operation {
    first: Option<u64>,
    second: Option<u64>,
    operator: Operator
}

impl Operation{
    fn calc(&self, input: u64) -> u64 {
        match self.operator {
            Operator::Add => {
                self.first.unwrap_or(input) + self.second.unwrap_or(input)
            },
            Operator::Mul => {
                self.first.unwrap_or(input) * self.second.unwrap_or(input)
            },

        }
    }

}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let first: Option<u64> = parts[0].parse::<u64>().ok();
        let second: Option<u64> = parts[2].parse::<u64>().ok();
        let operator = match parts[1] {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => unreachable!(),
        };
        Ok(Operation { first, second, operator })
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    friends: (usize,usize),
    inspect_count: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, test: u64, friends: (usize,usize)) -> Self {
        Monkey {
            items: items.into(),
            operation,
            test,
            friends,
            inspect_count: 0,
        }
    }

    fn inspect_next(&mut self) -> Option<(u64, usize)>{
        
        if let Some(mut item) = self.items.pop_front() {
            self.inspect_count += 1;
            item = self.operation.calc(item) / 3;
            
            if item % self.test as u64 == 0 {
                return Some((item, self.friends.0))
            } else {
                return Some((item, self.friends.1))
            }
        }
        None
    }

    fn inspect2_next(&mut self) -> Option<(u64, usize)>{
        
        if let Some(mut item) = self.items.pop_front() {
            self.inspect_count += 1;
            item = self.operation.calc(item);
            
            if item % self.test as u64 == 0 {
                return Some((item, self.friends.0))
            } else {
                return Some((item, self.friends.1))
            }
        }
        None
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey i:{:?}, f:{:?}, c:{}", self.items, self.friends, self.inspect_count )
    }
}


fn parse_block(block: &str) -> Monkey {
    let lines : Vec<&str> = block.lines().collect();
    let items = lines[1].split_once(":").unwrap().1.split(",").map(|x| x.trim().parse().unwrap()).collect();
    let operation_str = lines[2].split_once("=").unwrap().1;
    let operation = Operation::from_str(operation_str).unwrap();
    let test = lines[3].split_whitespace().last().unwrap().parse().unwrap();
    let f1 = lines[4].split_whitespace().last().unwrap().parse().unwrap();
    let f2 = lines[5].split_whitespace().last().unwrap().parse().unwrap();
    Monkey::new(items, operation, test, (f1,f2))
}

pub fn task1(path: &Path) -> u64 {
    let mut monkeys : Vec<Monkey> = get_input_by_block(path).iter().map(|b| parse_block(b)).collect();
    for _ in 1..21 {
        for idx in 0..monkeys.len() {
            while let Some((item,to)) = monkeys[idx].inspect_next() {
                monkeys[to].add_item(item);
            }
        }
    }

    monkeys.iter().map(|m| m.inspect_count ).sorted().rev().take(2).product::<u64>()
}

pub fn task2(path: &Path) -> u64 {
    let mut monkeys : Vec<Monkey> = get_input_by_block(path).iter().map(|b| parse_block(b)).collect();
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test).product();
    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            while let Some((item,to)) = monkeys[idx].inspect2_next() {
                monkeys[to].add_item(item % common_multiple);
            }
        }
    }

    monkeys.iter().map(|m| m.inspect_count ).sorted().rev().take(2).product::<u64>()
}

mod tests {
    
    use std::path::Path;

    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u64 = 0;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u64 = 0;

        assert_eq!(task2(path), expected_result)
    }
}