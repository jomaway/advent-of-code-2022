use std::{path::Path};

use super::get_input_by_line;

const DAY : u8 = 8; 

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
    let lines = get_input_by_line(path);
    let grid : Vec<Vec<u8>> = lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>() ).collect();
    let mut tree_count = 0;
    let rows = grid.len();
    for x in 0..grid.len() {
        let cols = grid[x].len();
        for y in 0..cols {
                if  check_visibility((0..y+1).rev().map(|i| grid[x][i]).collect()) || 
                    check_visibility((y..cols).map(|i| grid[x][i]).collect() ) ||
                    check_visibility((0..x+1).rev().map(|i| grid[i][y]).collect() ) ||
                    check_visibility((x..rows).map(|i| grid[i][y]).collect() ) 
                {
                    tree_count += 1;
                } 
        }
    }
    tree_count
}

fn check_tree_in_row(row: Vec<u8>, idx: usize) -> bool {
    // puhh this is not nearly nice code at all :D
    let split = row.split_at(idx);
    if split.0.iter().max().unwrap() < &row[idx] {
        return true;
    } else if split.1.iter().skip(1).max().unwrap() < &row[idx] {
        return true;
    } else {
        return false;
    }
}

fn check_visibility(view: Vec<u8>) -> bool {
    if view.len() == 1 { return true; }
    let mut iter = view.iter();
    if let Some(house) = iter.next() {
        return iter.max().unwrap_or(&0) < house;
    }
    false
}


pub fn task2(path: &Path) -> u32 {
    let lines = get_input_by_line(path);
    let grid : Vec<Vec<u8>> = lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>() ).collect();
    let mut tree_count = 0;
    let rows = grid.len();
    for x in 0..grid.len() {
        let cols = grid[x].len();
        for y in 0..cols {
            let left = check_view((0..y+1).rev().map(|i| grid[x][i]).collect() );
            let right = check_view((y..cols).map(|i| grid[x][i]).collect());
            let top = check_view((0..x+1).rev().map(|i| grid[i][y]).collect() );
            let bottom = check_view((x..rows).map(|i| grid[i][y]).collect() ); 
            let mul = left * right * top * bottom;
            //println!("x:{}, y:{}, left:{}, right:{}, top:{}, bottom:{}, mul:{}",x,y , left, right, top, bottom, mul);
            tree_count = if mul > tree_count { mul } else { tree_count }
        }
    }
    tree_count as u32
}


fn check_view(view: Vec<u8>) -> usize 
{
    if view.len() == 1 { return 0; }
    let mut iter = view.iter();
    if let Some(house) = iter.next() {
        let distance = iter.position(|t| *t >= *house).unwrap_or(view.len() - 2);
        distance + 1
    } else {
        1
    }
}

mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY, check_tree_in_row, check_view};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 21;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 8;

        assert_eq!(task2(path), expected_result)
    }

    #[test]
    fn test_check_view() {
        let row = vec![4,2,3, 2,1,2,3,4,3,1,0,2];
        assert_eq!(check_view(row), 7);
        let row = vec![4,2,3, 2,1,2,3,2,3,1,0,2];
        assert_eq!(check_view(row),11);
    }
}