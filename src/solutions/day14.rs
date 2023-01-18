use std::{path::Path, str::FromStr, num::ParseIntError, collections::HashSet};

use super::get_input_by_line;

const DAY : u8 = 14; 

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
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x,y)) = s.split_once(',') {
            return Ok(Point {
                x: x.trim().parse().expect("Parsing error"),
                y: y.trim().parse().expect("Parsing error"),
            })
        }
        Err("Parsing Error".to_string())
    }
}

impl Point {
    fn below(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    
    fn over(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

}

type Rock = Vec<Point>;

struct Sand {
    pos: Point,
}

impl Sand {
    fn new() -> Self {
        Sand {
            pos: Point { x: 500, y: 0 }
        }
    }

    // returns false if could not fall further
    fn fall(&mut self, grid: &HashSet<Point>) -> bool {
        let posibilities = vec![self.pos.below(), self.pos.below().left(), self.pos.below().right()];

        for next_pos in posibilities {
            if !grid.contains(&next_pos) {
                self.pos = next_pos;
                return true
            } 
        }
        false
    }
}

// Turning a vec of corner points into a solid line of rock
fn rocksolid (rocks: &Rock) -> Rock {
    rocks.windows(2)
        .flat_map(|window| generate_line_points(window[0], window[1]) )
        .collect()
}


fn generate_line_points(a: Point, b: Point) -> Vec<Point> {
    let x_min = std::cmp::min(a.x, b.x);
    let x_max = std::cmp::max(a.x, b.x);
    let y_min = std::cmp::min(a.y, b.y);
    let y_max = std::cmp::max(a.y, b.y);
    (x_min..=x_max)
        .flat_map(|x| (y_min..=y_max).map(move |y| Point { x, y }))
        .collect()
}

fn parse_input(path: &Path) -> HashSet<Point> {
    let lines = get_input_by_line(path);
    let rocks : Vec<Rock> = lines.iter().map(|line| 
        line.split("->").map(|point| 
            Point::from_str(point).expect("ParsingError")).collect::<Rock>()
        ).collect();

    let mut grid: HashSet<Point> = HashSet::new();

    for point in rocks.iter().flat_map(|rock| rocksolid(rock)) {
        // Add point to grid!
        grid.insert(point);
    }

    // Grid setup complete
    grid
}

fn print_grid(grid: &HashSet<Point>) {
    let y_max = grid.iter().map(|p| p.y ).max().unwrap();


    // Print Grid for debug purpose
    let x_min = grid.iter().map(|p| p.x ).min().unwrap();
    let x_max = grid.iter().map(|p| p.x ).max().unwrap();

    for x in (x_min..x_max) {
        if x == 500 { print!("|") }
        else { print!(" ")}
    }

    println!();

    for y in (0..y_max+1) {
        print!("{:3}", y);
        for x in (x_min..x_max +1) {
            if grid.contains(&Point {x,y}) { print!("#") }
            else { print!(" ")}
        }
        println!();
    }

    // END PRINT GRID
}

pub fn task1(path: &Path) -> u32 {
    let mut grid = parse_input(path);

    // get highest y value of the grid
    let y_max = grid.iter().map(|p| p.y ).max().unwrap();

    // counting units of sand
    let mut count = 0;

    println!("Let the sand fall ...");
    loop {
        let mut sand = Sand::new();
        while sand.fall(&grid) {
            // check if sand is falling into the nirvana.
            if sand.pos.y > y_max {
                return count;
            }
        }
        grid.insert(sand.pos);
        count += 1;
    }

    count
}


pub fn task2(path: &Path) -> u32 {
    let mut grid = parse_input(path);

    // get highest y value of the grid
    let y_max = grid.iter().map(|p| p.y ).max().unwrap();

    // Add bottom should be infiniv but i guess 0 to 1000 is enough to simulate it.
    for x in 0..1000 {
        grid.insert(Point {x, y: y_max +2});
    }

    // counting units of sand
    let mut count = 0;

    println!("Let the sand fall ...");
    loop {
        let mut sand = Sand::new();
        while sand.fall(&grid) {}
        // check if sand is falling into the nirvana.
        grid.insert(sand.pos);
        count += 1;
        
        if sand.pos == Sand::new().pos {
            return count;
        }
    }

    count
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