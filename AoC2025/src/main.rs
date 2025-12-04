use std::{env, fs};

mod days;

fn main() {
    // Read day argument, e.g.:
    // cargo run --release -- 1
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: aoc2025 <day-number>");
        return;
    }

    let day_num = args[1].parse::<u32>().expect("Day must be a number 1–25");

    // Load input file automatically
    let input_path = format!("input/day{:02}.txt", day_num);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Could not read {}", input_path));

    // Dispatch to the selected day's solver
    match day_num {
        1 => days::day01::run(&input),
        2 => days::day02::run(&input),
        3 => days::day03::run(&input),
        4 => days::day04::run(&input),
        // Add more as you go
        _ => eprintln!("Day {} not implemented yet", day_num),
    }
}
