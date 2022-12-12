use std::env;

mod solutions;

fn main() {
    println!("Running advent of code 22.");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // run all days
        println!("Run latest available solutions ...");
        solutions::day11::run();
    } else {
        // check given day
        let day = &args[1].parse::<u8>();

        match day {
            Ok(1) => solutions::day1::run(),
            Ok(2) => solutions::day2::run(),
            Ok(3) => solutions::day3::run(),
            Ok(4) => solutions::day4::run(),
            Ok(5) => solutions::day5::run(),
            Ok(6) => solutions::day6::run(),
            Ok(7) => solutions::day7::run(),
            Ok(8) => solutions::day8::run(),
            Ok(9) => solutions::day9::run(),
            Ok(10) => solutions::day10::run(),
            Ok(11) => solutions::day11::run(),
            Ok(day) => println!("Day {} not found", day),
            Err(_) => println!("Arg nees to be a valid day number."),
        }
    }
}
