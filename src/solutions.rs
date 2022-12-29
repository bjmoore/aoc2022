use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;

fn read_input(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    Ok(f.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
}

pub fn run_one(day: u8, input: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let input = match input {
        Some(input) => read_input(input)?,
        None => read_input(PathBuf::from(format!("input-{day}.txt")))?,
    };

    if let Some((part1, part2)) = match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        _ => None,
    } {
        println!("Day {day} Part 1: {part1}");
        println!("Day {day} Part 2: {part2}");
    }

    Ok(())
}

pub fn run_all() {
    for i in 1..=25 {
        run_one(i, None)
            .unwrap_or_else(|err| println!("Error running day {i}: {}", err.to_string()));
    }
}
