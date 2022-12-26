use std::{path::Path, vec, collections::{HashSet, BinaryHeap}, hash::Hash};

use super::get_input_by_line;

const DAY : u8 = 12; 

pub fn run() {
    let path_str = format!("src/inputs/day{}.txt",DAY);
    let path = Path::new(&path_str);

    println!("Running tasks of day {}", DAY);

    let res1 = task1(path);
    println!("Result of task1 is {}",res1);


    let res2 = task2(path);
    println!("Result of task2 is {}",res2);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new (x:usize, y:usize) -> Self {
        Pos {
            x,
            y,
        }
    }

    fn neighbours (&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut neigh = Vec::new();

        if self.x > 0 {
            neigh.push(Pos::new(self.x - 1, self.y));  // left
        }
        
        if self.y > 0 {
            neigh.push(Pos::new(self.x, self.y - 1));  // top
        }

        if self.y < rows - 1 {
            neigh.push(Pos::new(self.x, self.y + 1)); // bottom
        }

        if self.x < cols - 1 {
            neigh.push(Pos::new(self.x + 1, self.y))  // right
        }

        neigh
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: u32,
    pos: Pos,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn task1(path: &Path) -> u32 {
    // Parse input
    let mut start_pos = Pos::default();
    let mut end_pos = Pos::default();
    let grid : Vec<Vec<u8>> = get_input_by_line(path).iter().enumerate()
        .map(|(row, l)| {
            // convert input chars to height as integer
            l.chars().enumerate().map(|(col, c)| { 
                match c {
                    'S' => { start_pos = Pos::new(col, row); 0 },   // set start position and set height to 0
                    'E' => { end_pos = Pos::new(col, row); 25 },    // set end   position and set heigt to 25
                    'a'..='z' => c as u8 - b'a',            // convert chars to heigh from  a=0 to z=25
                    _ => unreachable!("invalid input"),
                }
            }).collect::<Vec<u8>>() 
        }).collect();
    // END Parse input
    
    let rows = grid.len();
    let cols = grid[0].len();

    let mut way: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    println!("START:{:?}, END:{:?}", start_pos, end_pos);

    way.push(Node{
        cost: 0,
        pos: start_pos,
    });

    while let Some(Node {cost, pos}) = way.pop() {
        if pos == end_pos {
            return cost
        }

        let current_height = grid[pos.y][pos.x];

        for nb in pos.neighbours(rows, cols).iter()
            .filter(|npos| grid[npos.y][npos.x] <= current_height + 1 )
        {
            if visited.insert(*nb) {
                way.push(Node {
                    cost: cost + 1,
                    pos: *nb
                });
            }
        }
    }
    
    panic!("Should have found an END point")
}


pub fn task2(path: &Path) -> u32 {
        // Parse input
        let mut start_pos = Pos::default();
        let mut end_pos = Pos::default();
        let grid : Vec<Vec<u8>> = get_input_by_line(path).iter().enumerate()
            .map(|(row, l)| {
                // convert input chars to height as integer
                l.chars().enumerate().map(|(col, c)| { 
                    match c {
                        'S' => { start_pos = Pos::new(col, row); 0 },   // set start position and set height to 0
                        'E' => { end_pos = Pos::new(col, row); 25 },    // set end   position and set heigt to 25
                        'a'..='z' => c as u8 - b'a',            // convert chars to heigh from  a=0 to z=25
                        _ => unreachable!("invalid input"),
                    }
                }).collect::<Vec<u8>>() 
            }).collect();
        // END Parse input
        
        let rows = grid.len();
        let cols = grid[0].len();
    
        let mut way: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited: HashSet<Pos> = HashSet::new();
    
    
        way.push(Node{
            cost: 0,
            pos: end_pos,
        });
    
        while let Some(Node {cost, pos}) = way.pop() {

            let current_height = grid[pos.y][pos.x];

            if current_height == 0 {
                return cost
            }

            for nb in pos.neighbours(rows, cols).iter()
                .filter(|npos| grid[npos.y][npos.x] >= current_height -1  )
            {
                if visited.insert(*nb) {
                    way.push(Node {
                        cost: cost + 1,
                        pos: *nb
                    });
                }
            }
        }
        
        panic!("Should have found an END point")
}

mod tests {
    use std::path::Path;
    use super::{task1, task2, DAY};

    #[test]
    fn test_task1(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 31;

        assert_eq!(task1(path), expected_result)
    }
    
    #[test]
    fn test_task2(){
        let path_str : String = format!("src/inputs/day{}-e.txt",DAY);
        let path = Path::new(&path_str);
        let expected_result: u32 = 29;

        assert_eq!(task2(path), expected_result)
    }
}