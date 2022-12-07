use std::{path::Path, str::{Lines}};

use super::{get_input};

const DAY : u8 = 7; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

pub fn task1(path: &Path) -> u32 {
    let input = get_input(path);
    let mut lines = input.lines();

    let mut sum = 0;

    dir_size(&mut lines, &mut sum);
    sum 
}

fn dir_size(lines: & mut Lines, sum: &mut u32) -> u32 {
    let mut size: u32 = 0;
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            _  if line.starts_with("$ ls") => { 
                size += dir_size(lines, sum);
            },
            _ => if let Ok(s) = line.split(' ').nth(0).unwrap().parse::<u32>() {
                    size += s;
                },
        }
    }
    *sum += if size < 100000 { size } else { 0 };
    size
}



pub fn task2(path: &Path) -> u32 {
    let input = get_input(path);
    let mut lines = input.lines();

    const FS_SIZE: u32 = 70000000;
    const UPDATE_SIZE: u32 = 30000000;
    let disk_usage= lines.clone().filter(|l| l.chars().nth(0).unwrap().is_numeric()).map(|l| l.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap()).fold(0, |mut sum, x| {sum += x; sum});
    let needed_space = UPDATE_SIZE - (FS_SIZE - disk_usage);
    let mut sum = FS_SIZE;
    free_space(&mut lines, &mut sum, needed_space);
    sum
}

fn free_space(lines: & mut Lines, sum: &mut u32, space: u32) -> u32 {
    let mut size: u32 = 0;
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            _  if line.starts_with("$ ls") => { 
                size += free_space(lines, sum, space);
            },
            _ => if let Ok(s) = line.split(' ').nth(0).unwrap().parse::<u32>() {
                    size += s;
                },
        }
    }
    if size >= space && size < *sum {
        *sum = size
    }
    size
}


mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 95437;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 24933642;

        assert_eq!(task2(path), expected_result)
    }
}