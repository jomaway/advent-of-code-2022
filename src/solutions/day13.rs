use std::{path::Path, str::FromStr};


use itertools::Itertools;

use crate::solutions::get_input_by_block;

use super::get_input_by_line;

const DAY : u8 = 13; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}



#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<_> = s.chars().collect();
        let (packet, rest) = parse_list(&c);
        if rest.len() != 0 { return Err(())}
        Ok(packet)
    }
}

fn parse_num(list : &[char]) -> (Packet, &[char]) {
    let mut i = 0;  // find index for where the number ends

    while i < list.len() && list[i].is_ascii_digit() {
        i+=1;
    }
    let num : u8 = list[..i].iter().rev().enumerate().map(|(idx,&c)| (c as u8 - b'0') * 10_u8.pow(idx as u32)).sum();

    (Packet::Num(num), &list[i..])
}

fn parse_list(list: &[char]) -> (Packet, &[char]) {
    // remove the starting [ 
    let mut list = &list[1..];
    let mut packets : Vec<Packet> = Vec::new();

    // until end of list 
    while list[0] != ']' {
        match list[0] {
            ',' => list = &list[1..], // skip
            '[' => { 
                // new list
                let (p,rest) = parse_list(&list);
                packets.push(p);
                list = rest;
            }, 
            _ => {
                // new num
                let (num, rest) = parse_num(&list);
                packets.push(num);
                list = rest;
            },
        }
    }

    // return the packet and remove ] from the remaining list
    (Packet::List(packets), &list[1..])
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => { a.cmp(b) },
            (Packet::List(a), Packet::List(b)) => { a.cmp(b) }
            (Packet::List(a), Packet::Num(b)) => { a.cmp(&vec![Packet::Num(*b)]) }
            (Packet::Num(a), Packet::List(b)) => { vec![Packet::Num(*a)].cmp(&b) }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn task1(path: &Path) -> u32 {
    let pairs: Vec<(Packet,Packet)> = get_input_by_block(path).iter().map(|b| {
        let packets: Vec<Packet> = b.lines().take(2).map(|s| Packet::from_str(s).expect("Parsing error")).collect();
        (packets[0].clone(), packets[1].clone())
    }).collect();

    pairs.iter().positions(|(left, right) | left < right).map(|idx| (idx + 1) as u32).sum()
}


pub fn task2(path: &Path) -> u32 {
    let pairs: Vec<(Packet,Packet)> = get_input_by_block(path).iter().map(|b| {
        let packets: Vec<Packet> = b.lines().take(2).map(|s| Packet::from_str(s).expect("Parsing error")).collect();
        (packets[0].clone(), packets[1].clone())
    }).collect();

    let mut packets : Vec<Packet> = get_input_by_line(path).iter().filter(|line| !line.trim().is_empty() ).map(|line| Packet::from_str(line).expect("Parsing error") ).collect();
    
    packets.push(Packet::from_str("[[2]]").unwrap());
    packets.push(Packet::from_str("[[6]]").unwrap());

    packets.sort();

    packets.iter().positions(|packet| *packet == Packet::from_str("[[2]]").unwrap() || *packet == Packet::from_str("[[6]]").unwrap() ).map(|idx| (idx + 1) as u32).product()
}

mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 13 ;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 140;

        assert_eq!(task2(path), expected_result)
    }
}