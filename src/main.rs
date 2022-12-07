use std::env;

mod solutions;

fn main() {
    println!("Running advent of code 22.");

    let args: Vec<String> = env::args().collect();
    println!("Args len = {}", args.len());
    if args.len() < 2 {
        // run all days
        println!("Run all available solutions ...");
        solutions::day1::run();
        solutions::day2::run();
        solutions::day3::run();
        solutions::day4::run();
        solutions::day5::run();
        solutions::day6::run();
        solutions::day7::run();
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
            Ok(day) => println!("Day {} not found", day),
            Err(_) => println!("Arg nees to be a valid day number."),
        }
    }
}
