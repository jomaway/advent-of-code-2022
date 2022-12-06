use std::{path::Path};

use itertools::Itertools;

use super::get_input;

pub fn run() {
    let path = Path::new("src/inputs/day6.txt");

    println!("Running tasks of day 6");

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

pub fn task1(path: &Path) -> u32 {
    let stream = get_input(path).chars().collect::<Vec<char>>();
    let mut window = stream.windows(4);

    for (idx,chunk) in window.enumerate() {
        if chunk.iter().all_unique() {
            return idx as u32 + 4;
        }
    }
    return 0;
}   


pub fn task2(path: &Path) -> u32 {
    let stream = get_input(path).chars().collect::<Vec<char>>();
    let mut window = stream.windows(14);

    for (idx,chunk) in window.enumerate() {
        if chunk.iter().all_unique() {
            return idx as u32 + 14;
        }
    }
    return 0;
}

mod tests {
    use std::path::Path;
    use super::{task1, task2};

    const PATH : &str = "src/inputs/day6-e.txt";
    
    #[test]
    fn test_task1(){
        let path = Path::new(PATH);
        let expected_result: u32 = 7;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path = Path::new(PATH);
        let expected_result: u32 = 19;

        assert_eq!(task2(path), expected_result)
    }
}