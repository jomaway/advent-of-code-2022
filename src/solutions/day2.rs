use std::path::Path;

use super::get_input_by_line;

pub fn run() {
    let path = Path::new("src/inputs/input2.txt");

    println!("Running tasks of day 2");

    let res1 = task1(path);
    println!("Result for task1 is {}",res1);
    
    let res2 = task2(path);
    println!("Result for task1 is {}",res2);
}

fn calc_game_score(game: &str) -> u32 {
    // Rock A || X
    // Paper B || Y
    // Scissor C || Z

    match game {
        // Oponent has Rock
        "A X" => 4, // 3 for Draw & 1 for Rock
        "A Y" =>  8, // 6 for Win & 2 for Paoer
        "A Z" => 3, // 0 for Lose & 3 for Scissor
        //Opponent has Paper
        "B X" => 1, // 0 for Lose & 1 for Rock
        "B Y" =>  5, // 3 for Draw & 2 for Paoer
        "B Z" => 9, // 6 for Win & 3 for Scissor
        // Opponent has Scissor
        "C X" => 7, // 6 for Win & 1 for Rock
        "C Y" =>  2, // 0 for Lose & 2 for Paoer
        "C Z" => 6, // 3 for Draw & 3 for Scissor
        _ => 0,
    }
}

fn calc_game_score2(game: &str) -> u32 {
    // Rock A 
    // Paper B 
    // Scissor C 
    // X= Need to Lose
    // Y = Draw
    // Z = Win

    match game {
        // Oponent has Rock
        "A X" => 3, // 0 for Lose  & 3 for Scissor
        "A Y" =>  4, // 3 for Draw & 1 for Rock
        "A Z" => 8, // 6 for Win & 2 for Paper
        //Opponent has Paper
        "B X" => 1, // 0 for Lose & 1 for Rock
        "B Y" =>  5, // 3 for Draw & 2 for Paoer
        "B Z" => 9, // 6 for Win & 3 for Scissor
        // Opponent has Scissor
        "C X" => 2, // 0 for Lose & 2 for Paoer 
        "C Y" =>  6, // 3 for Draw & 3 for Scissor
        "C Z" => 7,  // 6 for Win & 1 for Rock
        _ => 0,
    }
}

pub fn task1(path: &Path) -> u32 {
    let lines = get_input_by_line(path);
    let score = lines.iter().map(|g| calc_game_score(g)).sum();
    score
}

pub fn task2(path: &Path) -> u32 {
    let lines = get_input_by_line(path);
    let score = lines.iter().map(|g| calc_game_score2(g)).sum();
    score
}


mod test {
    use std::path::Path;
    use super::{task1, task2};

    #[test]
    fn test_task_1() {
        let path = Path::new("src/inputs/input2-e.txt");
        assert_eq!(task1(path), 15)
    }

    #[test]
    fn test_task_2() {
        let path = Path::new("src/inputs/input2-e.txt");
        assert_eq!(task2(path),12)
    }
}