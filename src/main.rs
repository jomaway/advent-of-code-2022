use std::env;

mod solutions;

fn main() {
    println!("Running advent of code 22.");

    let args : Vec<String> = env::args().collect();
    println!("Args len = {}", args.len());
    if args.len() < 2 { 
        // run all days
        println!("Run all available solutions ...");
        solutions::day1::run();

    } else {
        // check given day
        let day = &args[1].parse::<u8>();

        match day {
            Ok(1) => 
            solutions::day1::run(),
            Ok(day) => println!("Day {} not found", day),
            Err(_) => println!("Arg nees to be a valid day number.")
        }
    }
}
