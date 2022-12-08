use std::{path::Path};

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
    unimplemented!();
}


pub fn task2(path: &Path) -> u32 {
    unimplemented!();
}

mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 0;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 0;

        assert_eq!(task2(path), expected_result)
    }
}