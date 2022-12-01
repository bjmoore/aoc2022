use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = File::open("input-1.txt").unwrap();
    let f = BufReader::new(f);

    let mut elfs = vec![0; 3];
    let mut current_elf = 0;

    for line in f.lines() {
        let line = line.unwrap();
        if line == "" {
            if current_elf > elfs[0] {
                elfs[0] = current_elf;
                elfs.sort();
            }
            current_elf = 0;
        }
        else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {}", elfs[2]);
    println!("Part 2: {}", elfs.iter().sum::<i32>());
}