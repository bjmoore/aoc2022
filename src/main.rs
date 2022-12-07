use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::cell::RefCell;

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
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
        score_1 += strategy_values_1.get::<str>(&line).unwrap();
        score_2 += strategy_values_2.get::<str>(&line).unwrap();
    }

    println!("Day 2 Part 1: {}", score_1);
    println!("Day 2 Part 2: {}", score_2);
}

fn day_3() {
    let f = File::open("input-3.txt").unwrap();
    let f = BufReader::new(f);
    let priority_map: HashMap<char, i32> = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(0..)
        .collect();

    let mut priority_sum = 0;

    for chunk in &f.lines().chunks(3) {
        let shared_char = chunk
            .map(|line| line.unwrap().chars().collect::<HashSet<char>>())
            .reduce(|acc, hs| hs.intersection(&acc).copied().collect())
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone();

        priority_sum += priority_map.get(&shared_char).unwrap();
    }

    println!("Day 3 Part 1: {}", "NOT IMPLEMENTED");
    println!("Day 3 Part 2: {}", priority_sum);
}

fn day_4() {
    let f = File::open("input-4.txt").unwrap();
    let f = BufReader::new(f);

    let mut contained = 0;
    let mut overlap = 0;

    for line in f.lines() {
        let line = line.unwrap();

        let vals: Vec<u32> = line
            .split(&['-', ','])
            .map(|s| s.parse().unwrap())
            .collect();

        if vals[0] <= vals[2] && vals[1] >= vals[3] {
            contained += 1;
            overlap += 1;
        } else if vals[0] >= vals[2] && vals[1] <= vals[3] {
            contained += 1;
            overlap += 1;
        } else if (vals[0] >= vals[2] && vals[0] <= vals[3])
            || (vals[1] >= vals[2] && vals[1] <= vals[3])
        {
            overlap += 1;
        }
    }

    println!("Day 4 Part 1: {}", contained);
    println!("Day 4 Part 2: {}", overlap);
}

fn day_5() {
    let f = File::open("input-5.txt").unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let initial_stack: HashMap<u32, RefCell<Vec<char>>> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| line != " 1   2   3   4   5   6   7   8   9 ")
        .fold(HashMap::new(), |mut acc: HashMap<_, RefCell<Vec<char>>>, line| {
                let row = line.chars()
                    .skip(1)
                    .step_by(4)
                    .zip(1..);

                for (box_name, column) in row {
                    if box_name != ' ' {
                        if let Some(vec) = acc.get_mut(&column) {
                            vec.get_mut().push(box_name);
                        } else {
                            acc.insert(column, RefCell::new(Vec::from([box_name])));
                        }
                    }
                }

                acc
        });

    for (_, stack) in initial_stack.iter() {
        stack.borrow_mut().reverse();
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        let line = line.unwrap();
        if let Some(cap) = re.captures(&line) {
            let count: usize = cap[1].parse().unwrap();
            let source = cap[2].parse().unwrap();
            let target = cap[3].parse().unwrap();
            let mut source = initial_stack.get(&source).unwrap().borrow_mut();
            let mut target = initial_stack.get(&target).unwrap().borrow_mut();
            let at: usize = source.len() - count;
            let mut moving_boxes = source.split_off(at);
            target.append(&mut moving_boxes);
        }
    }

    for i in 1..=9 {
        println!("{}", initial_stack.get(&i).unwrap().borrow().last().unwrap());
    }

    println!("Day 5 Part 1: {}", "NOT IMPLEMENTED");
    println!("Day 5 Part 2: {}", "NOT IMPLEMENTED");
}
