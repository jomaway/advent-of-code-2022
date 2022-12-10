use std::{path::Path};

use super::get_input_by_line;

const DAY : u8 = 10; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    task2(path);
    println!("Result of task2 is {}","done");
}

pub fn task1(path: &Path) -> i32 {
    let lines = get_input_by_line(path);
    let mut signal_strength = 0;
    let mut cycles = 0;
    let mut register = 1;

    for line in lines {
        if let Some((_, value)) = line.split_once(' ') {
            let value: i32 = value.parse().unwrap();

            cycles += 1;
            signal_strength += check_cycles(cycles, register);
            cycles += 1;
            signal_strength += check_cycles(cycles, register);
            register += value;
            println!("Reg: {} , val: {}", register, value);
        } else {
            // noop
            cycles += 1;
            signal_strength += check_cycles(cycles, register);
        }

    }

    signal_strength as i32
}

fn check_cycles(cycle: i32, reg_val: i32) -> i32 {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        println!("cycle {} - res {}", cycle, reg_val);
        return cycle * reg_val
    } 
    0
}

pub fn task2(path: &Path)  {
    let lines = get_input_by_line(path);
    let mut cycles = 0;
    let mut register = 1; // sprite position

    for line in lines {
        if let Some((_, value)) = line.split_once(' ') {
            let value: i32 = value.parse().unwrap();

            cycles += 1;
            print_crt(cycles, register);
            cycles += 1;
            print_crt(cycles, register);
            register += value;
        } else {
            // noop
            cycles += 1;
            print_crt(cycles, register);
        }

    }
}

fn print_crt(cycle: i32, reg_val: i32) {
    //print!("({}-{})",cycle, reg_val);
    if (reg_val-1..reg_val+2).contains(&((cycle-1)%40)) {
        print!("#")
    } else {
        print!(".")
    }
    if cycle % 40 == 0 {
        println!();
    }
    
}

mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: i32 = 13140;

        assert_eq!(task1(path), expected_result)
    }
}