use core::panic;
use std::{path::Path, collections::HashSet, ops::RangeInclusive};

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

    fn get_scanned_area_in_row(&self, row: i32) -> Option<RangeInclusive<i32>>
    {
        let dist = self.get_beacon_distance();
        let offset = dist - (self.pos.y - row).abs();
        if offset < 0 {
            None
        } else {
            Some(self.pos.x - offset..=self.pos.x + offset)
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


fn get_tuning_frequency(x:i32, y:i32) -> i64 {
    x as i64 * 4_000_000 + y as i64
}

fn row_range(row: i32, sensors: &Vec<Sensor>) -> Vec<RangeInclusive<i32>> {
    let mut ranges : Vec<_> = sensors.iter().flat_map(|s| s.get_scanned_area_in_row(row)).collect();

    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
   
    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if the two sorted ranges overlap
        if next.start() <= last.end() || last.end() + 1 == *next.start() {
            // replace last with a single bigger range if possible
            if next.end() > last.end() {
                let old = &merged_ranges[last_idx];
                let new = *old.start()..=*next.end();
                merged_ranges[last_idx] = new;
            }
        } else {
            // add to the ranges for this row
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges

}

pub fn task2(path: &Path) -> i64 {
    let sensors = parse_input(path);
    let max = 4_000_000;

    let (row, col_range) = (0..=max).rev()
        .map(|row| (row, row_range(row, &sensors)))
        .find(|(_,ranges)| ranges.len()  > 1)
        .unwrap();
    
    let col = col_range.first().unwrap().end() + 1;

    get_tuning_frequency(col, row) as i64
}

