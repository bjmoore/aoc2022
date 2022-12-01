use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = File::open("input-1.txt").unwrap();
    let f = BufReader::new(f);

    let mut elfs = Vec::new();
    let mut current_elf = 0;

    for line in f.lines() {
        let line = line.unwrap();
        if line == "" {
            elfs.push(current_elf);
            current_elf = 0;
        }
        else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    elfs.sort();
    elfs.reverse();

    let top_3: i32 = elfs[..3].iter().sum();

    println!("The top 3 elfs with the most calories are carrying {:?} calories", top_3);
}
