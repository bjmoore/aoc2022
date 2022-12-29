use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: BufReader<File>) -> Option<(i32, i32)> {
    let mut elfs = vec![0; 3];
    let mut current_elf = 0;

    for line in input.lines() {
        let line = line.unwrap();
        if line == "" {
            if current_elf > elfs[0] {
                elfs[0] = current_elf;
                elfs.sort();
            }
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    Some((elfs[2], elfs.iter().sum::<i32>()))
}
