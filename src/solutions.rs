use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;

fn read_input(path: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    f.lines()
        .into_iter()
        .collect::<Result<Vec<String>, std::io::Error>>()
}

pub fn run_one(day: u8, input: Option<PathBuf>) -> Result<(String, String), Box<dyn Error>> {
    let input = match input {
        Some(input) => read_input(input)?,
        None => read_input(PathBuf::from(format!("input-{day}.txt")))?,
    };

    match day {
        1 => day1::solve(input),
        //2 => day2::solve(input),
        //3 => day3::solve(input),
        //4 => day4::solve(input),
        _ => Err(Box::from(format!("Day {day} not implemented"))),
    }
}
