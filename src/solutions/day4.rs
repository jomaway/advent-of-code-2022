use std::{path::Path};

use itertools::Itertools;

use crate::solutions::get_input_by_line;

pub fn run() {
    let path = Path::new("src/inputs/input4.txt");

    println!("Running tasks of day 4");

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

fn is_fully_overlap(a1: u32,a2:u32,b1:u32,b2:u32) -> bool {
    if a1 >=b1 && a2 <= b2 { return true; }
    else if b1 >= a1 && b2 <= a2 {
        return true;
    }
    return false;
}

fn is_overlap(a1: u32,a2:u32,b1:u32,b2:u32) -> bool {
    if is_fully_overlap(a1, a2, b1, b2) {return true;}
    if a1 <= b1 && a2 >= b1 {return true; }
    if a1 <= b2 && a2 >= b2 {return  true; }
    false
}

pub fn task1(path: &Path) -> u32 {
    let pairs = get_input_by_line(path);
    let mut nr_of_fully_overlaps:u32 = 0;
    
    for pair in pairs {
        let (a,b)= pair.split(",").collect_tuple().unwrap();
        let (a1,a2) = a.split("-").map(|c| c.parse::<u32>().unwrap()).collect_tuple().unwrap();
        let (b1,b2) = b.split("-").map(|c| c.parse::<u32>().unwrap()).collect_tuple().unwrap();
        if is_fully_overlap(a1, a2, b1, b2) {
            nr_of_fully_overlaps += 1;
        }
    }
    
    return nr_of_fully_overlaps;
}


pub fn task2(path: &Path) -> u32 {
    let pairs = get_input_by_line(path);
    let mut nr_of_overlaps:u32 = 0;
    
    for pair in pairs {
        let (a,b)= pair.split(",").collect_tuple().unwrap();
        let (a1,a2) = a.split("-").map(|c| c.parse::<u32>().unwrap()).collect_tuple().unwrap();
        let (b1,b2) = b.split("-").map(|c| c.parse::<u32>().unwrap()).collect_tuple().unwrap();
        if is_overlap(a1, a2, b1, b2) {
            nr_of_overlaps += 1;
        }
    }
    
    return nr_of_overlaps;
}

mod tests {
    use std::path::Path;
    use super::{task1, task2};

    const PATH : &str = "src/inputs/input4-e.txt";
    
    #[test]
    fn test_task1(){
        let path = Path::new(PATH);
        let expected_result: u32 = 2;
        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path = Path::new(PATH);
        let expected_result: u32 = 4;

        assert_eq!(task2(path), expected_result)
    }
}