use std::error::Error;
use std::fs::File;
use std::io;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn run_one(day: u8) -> Result<(), Box<dyn Error>> {
    let f = File::open(format!("input-{day}.txt"))?;
    let f = io::BufReader::new(f);

    if let Some((part1, part2)) = match day {
        1 => day1::solve(f),
        2 => day2::solve(f),
        3 => day3::solve(f),
        4 => day4::solve(f),
        _ => None,
    } {
        println!("Day {day} Part 1: {part1}");
        println!("Day {day} Part 2: {part2}");
    }

    Ok(())
}

pub fn run_all() {
    for i in 1..=25 {
        run_one(i).unwrap_or_else(|err| println!("Error running day {i}: {}", err.to_string()));
    }
}
