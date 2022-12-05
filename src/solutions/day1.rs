use std::path::Path;

use super::get_input_by_block;

pub fn run() {
    let path = Path::new("src/inputs/input1.txt");

    println!("Running tasks of day 1");

    let res1 = task1(path);
    println!("Result of task1 is {}", res1);

    let res2 = task2(path);
    println!("Result of task2 is {}", res2);
}

pub fn task1(path: &Path) -> u32 {
    let blocks = get_input_by_block(path);
    let amounts = blocks
        .iter()
        .map(|b| b.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    let biggest = amounts.iter().max().unwrap();
    *biggest
}

pub fn task2(path: &Path) -> u32 {
    let blocks = get_input_by_block(path);
    let mut amounts = blocks
        .iter()
        .map(|b| b.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    amounts.sort();
    let top_three =
        amounts[amounts.len() - 1] + amounts[amounts.len() - 2] + amounts[amounts.len() - 3];
    top_three
}

mod tests {
    use super::{task1, task2};
    use std::path::Path;

    #[test]
    fn test_task1() {
        let path = Path::new("src/inputs/input1-e.txt");
        assert_eq!(task1(path), 24000)
    }

    #[test]
    fn test_task2() {
        let path = Path::new("src/inputs/input1-e.txt");
        assert_eq!(task2(path), 45000)
    }
}
