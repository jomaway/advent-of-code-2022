use std::{path::Path};

use super::get_input_by_line;

pub fn run() {
    let path = Path::new("src/inputs/input3.txt");

    println!("Running tasks of day 3");

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

fn char_to_value(c: char) -> u32 {
    let n = c as u32;
    
    if n >= 97 && n <= 122 { n-96 }  // a-z to 1-26
    else if n >= 65 && n <= 90 { n-38 } // A-Z  to 27-52
    else {0}
}

pub fn task1(path: &Path) -> u32 {
    let rucksacks = get_input_by_line(path);

    println!("Rucksacks: {}", rucksacks.len()); 
    let mut sum : u32 = 0;

    for rucksack in rucksacks {
        let (comp_a, comp_b) = rucksack.split_at(rucksack.len()/2);
        for c in comp_a.chars() {
            if comp_b.contains(c) {
                sum += char_to_value(c);
                break;
            }
        }
    }    

    return sum;
}


pub fn task2(path: &Path) -> u32 {
    let rucksacks = get_input_by_line(path);
    let mut sum: u32 =  0;

    for group in rucksacks.chunks(3) {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                sum += char_to_value(c);
                break;
            }
        }
    }

    return sum;
}

mod tests {
    use std::path::Path;
    use super::{task1, task2};

    const PATH : &str = "src/inputs/input3-e.txt";
    
    #[test]
    fn test_task1(){
        let path = Path::new(PATH);
        assert_eq!(task1(path), 157)
    }
    
    #[test]
    fn test_task2(){
        let path = Path::new(PATH);
        assert_eq!(task2(path), 70)
    }
}