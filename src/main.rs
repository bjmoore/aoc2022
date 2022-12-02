use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
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

    println!("Day 1 Part 1: {}", elfs[2]);
    println!("Day 1 Part 2: {}", elfs.iter().sum::<i32>());
}

fn day_2() {

}