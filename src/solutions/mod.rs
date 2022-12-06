use std::{fs, path::Path};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn get_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).expect("Error reading input file")
}

pub fn get_input_by_line<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    get_input(path)
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn get_input_by_block<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    get_input(path)
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn get_input_by_line_chunks<P>(path: P, chunk_size: usize) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    get_input_by_line(path)
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<String>>>()
}
