use std::{path::{Path, PathBuf}, fs};

pub mod day1;

pub fn get_input<P>(path: P) -> String
where P: AsRef<Path>
{
    fs::read_to_string(path).expect("Error reading input file")
}

pub fn get_input_by_line<P>(path: P) -> Vec<String>
where P: AsRef<Path>
{
    get_input(path).lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

pub fn get_input_by_block<P>(path: P) -> Vec<String>
where P: AsRef<Path> 
{
    get_input(path).split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>()
}


