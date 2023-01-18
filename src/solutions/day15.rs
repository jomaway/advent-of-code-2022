use core::panic;
use std::{path::Path, collections::HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::solutions::get_input_by_line;

const DAY : u8 = 15; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_all_points_within_distance_of(&self, dist: i32) -> Vec<Point> {
        let x_min = self.x - dist;
        let x_max = self.x + dist;
        let y_max = self.y + dist;
        let y_min = self.y - dist;

        (x_min..=x_max).flat_map(|x| (y_min..=y_max).map(move |y| Point { x, y }))
        .filter(|p| self.manhattan_distance(p) <= dist )
        .collect()
    }
}

struct Sensor {
    pos: Point,
    beacon: Point,
}

impl Sensor {
    // returns the distance to the closest beacon
    fn get_beacon_distance(&self) -> i32 
    {
        self.pos.manhattan_distance(&self.beacon)
    }

    fn get_scanned_area_in_row(&self, row: i32) -> Option<Vec<i32>>
    {
        let dist = self.get_beacon_distance();
        let offset = dist - (self.pos.y - row).abs();
        if offset < 0 {
            None
        } else {
            Some((self.pos.x - offset..=self.pos.x + offset).collect())
        }
    }
}

fn extract_numbers(s: &str) -> Vec<i32> {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut numbers = vec![];
    for cap in re.captures_iter(s) {
        numbers.push(cap[0].parse().unwrap());
    }
    numbers
}

fn parse_input(path: &Path) -> Vec<Sensor>{
    let lines = get_input_by_line(path);
    lines.iter().map(|line| {
        let numbers = extract_numbers(line);
        if numbers.len() != 4 { panic!("Should not have been happened")};
        //println!("Numbers: {:?}", numbers);
        // returning a Sensor
        Sensor {
            pos: Point { x: numbers[0], y: numbers[1] },
            beacon: Point { x: numbers[2], y: numbers[3] },
        }
    }).collect()
}

pub fn task1(path: &Path) -> u32 {
    let sensors = parse_input(path);
    let row = 2_000_000;

    println!("Parsing complete");
    let mut grid: HashSet<i32> = HashSet::new();

    let beacons: Vec<i32> = sensors.iter().map(|s| s.beacon.y).filter(|y| *y == row).dedup().collect();
    
    for v in sensors.iter().filter_map(|sensor| sensor.get_scanned_area_in_row(row)) {
        for i in v {
            grid.insert(i);
        }
    }

    grid.len() as u32 - beacons.len() as u32
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