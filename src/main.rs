use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    println!("Day 1 Part 1: {}", elfs[2]);
    println!("Day 1 Part 2: {}", elfs.iter().sum::<i32>());
}

fn day_2() {
    let f = File::open("input-2.txt").unwrap();
    let f = BufReader::new(f);

    let mut score_1 = 0;
    let strategy_values_1 = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let mut score_2 = 0;
    let strategy_values_2 = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    for line in f.lines() {
        let line = line.unwrap();
        score_1 += strategy_values_1.get(&*line).unwrap();
        score_2 += strategy_values_2.get(&*line).unwrap();
    }

    println!("Day 2 Part 1: {}", score_1);
    println!("Day 2 Part 2: {}", score_2);
}
