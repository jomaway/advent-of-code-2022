use std::{path::Path, collections::HashSet, fmt::Display};

use crate::solutions::get_input_by_line;

const DAY : u8 = 9; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn one_left(&mut self) {
        self.x -= 1;
    }
    fn one_right(&mut self) {
        self.x += 1;
    }
    fn one_up(&mut self) {
        self.y += 1;
    }
    fn one_down(&mut self) {
        self.y -= 1;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})",self.x, self.y)
    }
}


pub fn task1(path: &Path) -> u32 {
    let instructions = get_input_by_line(path);
    let mut head = Position::default();
    let mut tail = head;
    let mut history: HashSet<Position> = HashSet::new();
    history.insert(tail);
    
    for line in instructions {
        let (direction, amount) = line.split_once(' ').expect("Couldn't parse input line");
        let amount = amount.parse().unwrap();
        for _ in 0..amount {
            let last = head.clone();
            match direction {
                "L" => head.one_left(),
                "R" => head.one_right(),
                "U" => head.one_up(),
                "D" => head.one_down(),
                _ => unreachable!(),
            }

            let delta_x = head.x - tail.x;
            let delta_y = head.y - tail.y;
            let dis = delta_x.pow(2) + delta_y.pow(2);
            
            if dis > 2 {
                tail = last;
                history.insert(tail);
            }
        }
    }
    history.len() as u32
}

pub fn task2(path: &Path) -> u32 {
    let instructions = get_input_by_line(path);
    let mut rope : Vec<Position> = vec![Position::default(); 10];
    let mut history: HashSet<Position> = HashSet::new();
    history.insert(rope[9]);
    
    for line in instructions {
        let (direction, amount) = line.split_once(' ').expect("Couldn't parse input line");
        let amount = amount.parse().unwrap();
        for _ in 0..amount {
            let mut last = rope[0].clone();
            match direction {
                "L" => rope[0].one_left(),
                "R" => rope[0].one_right(),
                "U" => rope[0].one_up(),
                "D" => rope[0].one_down(),
                _ => unreachable!(),
            }

            for knot in 1..rope.len() {
                let delta_x = rope[knot-1].x - rope[knot].x;
                let delta_y = rope[knot-1].y - rope[knot].y;
                let dis = delta_x.pow(2) + delta_y.pow(2);
            
                if dis > 2 {
                    rope[knot].x += delta_x.signum();
                    rope[knot].y += delta_y.signum();
                }

            }
            println!("Tail at: {}", rope[9]);
            history.insert(rope[9]);
        }
    }
    history.len() as u32
}


mod tests { 
    use std::path::Path;

    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 13;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e2.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 36;

        assert_eq!(task2(path), expected_result)
    }

}