use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = File::open("input-1.txt").unwrap();
    let f = BufReader::new(f);

    let mut max_elf = 0;
    let mut current_elf = 0;

    for line in f.lines() {
        let line = line.unwrap();
        if line == "" {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
        }
        else {
            current_elf += line.parse::<i32>().unwrap();
        }

    }

    println!("The elf with the most calories is carrying {} calories", max_elf);
}
