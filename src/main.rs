use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use crate::day1::solve;

mod day1;

fn main() {
    day1::solve();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
    day_7();
    day_8();
    day_10();
    day_11();
    day_12();
    day_13();
    day_14();
    day_18();
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

    let mut priority_total_area = 0;

    for chunk in &f.lines().chunks(3) {
        let shared_char = chunk
            .map(|line| line.unwrap().chars().collect::<HashSet<char>>())
            .reduce(|acc, hs| hs.intersection(&acc).copied().collect())
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone();

        priority_total_area += priority_map.get(&shared_char).unwrap();
    }

    println!("Day 3 Part 1: {}", "NOT IMPLEMENTED");
    println!("Day 3 Part 2: {}", priority_total_area);
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
        .fold(
            HashMap::new(),
            |mut acc: HashMap<_, RefCell<Vec<char>>>, line| {
                let row = line.chars().skip(1).step_by(4).zip(1..);

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
            },
        );

    for (_, stack) in initial_stack.iter() {
        stack.borrow_mut().reverse();
    }

    let filesize_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        let line = line.unwrap();
        if let Some(cap) = filesize_regex.captures(&line) {
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
        //println!("{}", initial_stack.get(&i).unwrap().borrow().last().unwrap());
    }

    println!("Day 5 Part 1: {}", "NOT IMPLEMENTED");
    println!("Day 5 Part 2: {}", "NOT IMPLEMENTED");
}

fn day_6() {
    let f = read_to_string("input-6.txt").unwrap();

    let mut buf: VecDeque<char> = VecDeque::new();
    let mut set: HashMap<char, u32> = HashMap::new();
    let mut first_packet_index = 0;

    for (i, c) in f.chars().enumerate() {
        if buf.len() == 14 {
            let c = buf.pop_back().unwrap();
            let mut count = set.get_mut(&c).unwrap();
            *count -= 1;
        }
        buf.push_front(c);
        if let Some(count) = set.get_mut(&c) {
            *count += 1;
        } else {
            set.insert(c, 1);
        }
        if buf.len() == 14 {
            if set.iter().all(|(_, &v)| v < 2) && set.iter().fold(0, |acc, (_, &v)| acc + v) == 14 {
                first_packet_index = i;
                break;
            }
        }
    }

    println!("Day 6 Part 1: {}", first_packet_index + 1);
    println!("Day 6 Part 2: {}", "NOT IMPLEMENTED");
}

fn day_7() {
    let f = File::open("input-7.txt").unwrap();
    let f = BufReader::new(f);

    let mut total_area_under_100000 = 0;
    let mut stack = Vec::new();
    let mut dir_sizes = Vec::new();
    let mut current_size: u32 = 0;
    let mut total_used: u32 = 0;

    let filesize_regex = Regex::new(r"^(\d+)").unwrap();

    for line in f.lines() {
        let line = line.unwrap();
        // if $ cd /, $ ls, dir xyz: ignore
        if let Some(cap) = filesize_regex.captures(&line) {
            // if 1234 x.txt: add to current dir size
            let filesize = cap[1].parse::<u32>().unwrap();
            total_used += filesize;
            current_size += filesize;
        } else if line == "$ cd .." {
            // if cd ..: add current size to total_area_under_100k if <100000, pop from stack and add to parent dir size
            dir_sizes.push(current_size);
            if current_size < 100000 {
                total_area_under_100000 += current_size;
            }
            current_size += stack.pop().unwrap();
        } else if line.starts_with("$ cd") {
            // if cd xyz: push current size to stack
            stack.push(current_size);
            current_size = 0;
        }
    }

    // at the end we need to pop our way back up the stack:

    dir_sizes.push(current_size);
    if current_size < 100000 {
        total_area_under_100000 += current_size;
    }

    while let Some(size) = stack.pop() {
        current_size += size;
        dir_sizes.push(current_size);
        if current_size < 100000 {
            total_area_under_100000 += current_size;
        }
    }

    let space_needed = total_used - 40000000;

    println!("Day 7 Part 1: {}", total_area_under_100000);
    println!(
        "Day 7 Part 2: {}",
        dir_sizes
            .iter()
            .filter(|x| *x > &space_needed)
            .min()
            .unwrap()
    );
}

fn day_8() {
    let f = File::open("8-test.txt").unwrap();
    let f = BufReader::new(f);

    let map: HashMap<(u8, u8), u32> = f
        .lines()
        .map(|l| l.unwrap())
        .zip(0..)
        .flat_map(|(l, i)| {
            l.chars()
                .zip(0..)
                .map(|(c, j)| ((i, j), c.to_digit(10).unwrap() + 1))
                .collect::<Vec<((u8, u8), u32)>>()
        })
        .collect();

    const max_len: u8 = 5;
    let mut scenic_score = [[1u32; max_len as usize]; max_len as usize];

    let mut vis = HashSet::new();
    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut last = 0;
        let mut scenic = 1;
        for j in 0..max_len {
            let height = *map.get(&(i, j)).unwrap();
            if height > vis_threshold {
                vis.insert((i, j));
                vis_threshold = height;
            }
            if height <= last {
                scenic = 1;
            } else {
                scenic += 1;
            }
            last = height;
            scenic_score[i as usize][j as usize] *= scenic;
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut last = 0;
        let mut scenic = 1;
        for j in 0..max_len {
            let height = *map.get(&(j, i)).unwrap();
            if height > vis_threshold {
                vis.insert((j, i));
                vis_threshold = height;
            }
            if height <= last {
                scenic = 1;
            } else {
                scenic += 1;
            }
            last = height;
            scenic_score[j as usize][i as usize] *= scenic;
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut last = 0;
        let mut scenic = 1;
        for j in (0..max_len).rev() {
            let height = *map.get(&(i, j)).unwrap();
            if height > vis_threshold {
                vis.insert((i, j));
                vis_threshold = height;
            }
            if height <= last {
                scenic = 1;
            } else {
                scenic += 1;
            }
            last = height;
            scenic_score[i as usize][j as usize] *= scenic;
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut last = 0;
        let mut scenic = 1;
        for j in (0..max_len).rev() {
            let height = *map.get(&(j, i)).unwrap();
            if height > vis_threshold {
                vis.insert((j, i));
                vis_threshold = height;
            }
            if height <= last {
                scenic = 1;
            } else {
                scenic_score[j as usize][i as usize] *= scenic;
                scenic += 1;
            }
            last = height;
        }
    }

    println!("Day 8 Part 1: {}", vis.len());
    println!("Day 8 Part 2: {:?}", scenic_score);
    println!(
        "Day 8 Part 2: {:?}",
        scenic_score.iter().flatten().max().unwrap()
    );
}

fn day_9() {
    let f = File::open("input-9.txt").unwrap();
    let f = BufReader::new(f);

    let mut tail_positions = HashSet::<(i32, i32)>::new();

    for line in f.lines() {
        let line = line.unwrap();

        // parse line
        // update HEAD position
        // while not_touching(head, tail) {
        //  if tail, head same row or column:
        //      move tail 1 toward head
        //  else:
        //      move tail toward head on each axis
        //  add tail pos to tail_positions
    }
}

// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the total_area of these six signal strengths?
fn day_10() {
    let f = File::open("input-10.txt").unwrap();
    let f = BufReader::new(f);

    let mut program = Vec::<i32>::new();
    for line in f.lines() {
        let line = line.unwrap();
        program.push(0);
        if line != "noop" {
            program.push(line[5..].parse::<i32>().unwrap());
        }
    }

    let state: Vec<i32> = program
        .iter()
        .scan(1, |state, &x| {
            *state += x;

            Some(*state)
        })
        .collect();

    let signal_total_area = state[19] * 20
        + state[59] * 60
        + state[99] * 100
        + state[139] * 140
        + state[179] * 180
        + state[219] * 220;

    let graphics: String = (1..)
        .zip(state.iter())
        .map(|(i, s)| {
            if (i % 40 <= *s + 1) && (i % 40 >= *s - 1) {
                '#'
            } else {
                ' '
            }
        })
        .collect();

    println!("Day 10 Part 1: {}", signal_total_area);
    println!("Day 10 Part 2: '{}'", graphics);
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    true_target: usize,
    false_target: usize,
    items_inspected: u64,
}

impl Monkey {
    fn inspect_and_throw(&mut self) -> (Vec<u64>, Vec<u64>) {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for item in self.items.iter() {
            self.items_inspected += 1;
            let item = (self.operation)(*item) % 9699690;
            if (self.test)(item) {
                true_items.push(item);
            } else {
                false_items.push(item);
            }
        }

        self.items.clear();
        (true_items, false_items)
    }

    fn new<O: 'static, T: 'static>(
        items: Vec<u64>,
        operation: O,
        test: T,
        true_target: usize,
        false_target: usize,
    ) -> Self
    where
        O: Fn(u64) -> u64,
        T: Fn(u64) -> bool,
    {
        Self {
            items,
            operation: Box::new(operation),
            test: Box::new(test),
            true_target,
            false_target,
            items_inspected: 0,
        }
    }
}

fn day_11() {
    let mut monkeys = Vec::new();
    monkeys.push(RefCell::new(Monkey::new(
        vec![63, 57],
        |x| x * 11,
        |x| x % 7 == 0,
        6,
        2,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![82, 66, 87, 78, 77, 92, 83],
        |x| x + 1,
        |x| x % 11 == 0,
        5,
        0,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![97, 53, 53, 85, 58, 54],
        |x| x * 7,
        |x| x % 13 == 0,
        4,
        3,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![50],
        |x| x + 3,
        |x| x % 3 == 0,
        1,
        7,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![64, 69, 52, 65, 73],
        |x| x + 6,
        |x| x % 17 == 0,
        3,
        7,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![57, 91, 65],
        |x| x + 5,
        |x| x % 2 == 0,
        0,
        6,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![67, 91, 84, 78, 60, 69, 99, 83],
        |x| x * x,
        |x| x % 5 == 0,
        2,
        4,
    )));
    monkeys.push(RefCell::new(Monkey::new(
        vec![58, 78, 69, 65],
        |x| x + 7,
        |x| x % 19 == 0,
        5,
        1,
    )));

    for _ in 0..10000 {
        for monkey in &monkeys {
            let mut main_monkey = monkey.borrow_mut();
            let mut true_monkey = monkeys[main_monkey.true_target].borrow_mut();
            let mut false_monkey = monkeys[main_monkey.false_target].borrow_mut();

            let (mut true_items, mut false_items) = main_monkey.inspect_and_throw();
            true_monkey.items.append(&mut true_items);
            false_monkey.items.append(&mut false_items);
        }
    }

    let mut monkey_business: Vec<u64> =
        monkeys.iter().map(|x| x.borrow().items_inspected).collect();
    monkey_business.sort();

    println!("Day 11 Part 2: {}", monkey_business[6] * monkey_business[7]);
}

fn day_12() {
    let height_values: HashMap<char, u8> = "abcdefghijklmnopqrstuvwxyz".chars().zip(1..).collect();

    // processing queue is a vecdeque
    // visited elements is a hashmap<(u8, u8), u8> (?? too small?)
    // goal point is a (u8, u8)

    // input is 171x40
    let f = File::open("input-12.txt").unwrap();
    let f = BufReader::new(f);

    let mut arr = [[1u8; 41]; 171];

    let mut visited: HashSet<(u8, u8)> = HashSet::new();
    let mut process_queue: VecDeque<(u8, u8, u16)> = VecDeque::new();
    let mut starting_point: (u8, u8) = (0, 0);

    for (line, y) in f.lines().zip(0..) {
        let line = line.unwrap();
        for (c, x) in line.chars().zip(0..) {
            if c == 'S' {
                starting_point = (x, y);
            } else if c == 'E' {
                arr[x as usize][y as usize] = 26;
                process_queue.push_back((x, y, 0));
                visited.insert((x, y));
            } else {
                arr[x as usize][y as usize] = *height_values.get(&c).unwrap();
            }
        }
    }

    let mut shortest_path_to_start = 0;
    let mut shortest_path_to_a = 10000;

    while let Some((x, y, dist)) = process_queue.pop_front() {
        let height = arr[x as usize][y as usize];
        let mut to_visit = Vec::<(u8, u8)>::new();
        if x > 0 {
            to_visit.push((x - 1, y));
        }
        if x < 170 {
            to_visit.push((x + 1, y));
        }
        if y > 0 {
            to_visit.push((x, y - 1));
        }
        if y < 40 {
            to_visit.push((x, y + 1));
        }

        for (i, j) in to_visit {
            let target_height = arr[i as usize][j as usize];
            if visited.contains(&(i, j)) {
                continue;
            }

            if target_height >= height - 1 {
                if (i, j) == starting_point {
                    shortest_path_to_start = dist + 1;
                } else if target_height == 1 && shortest_path_to_a > dist + 1 {
                    shortest_path_to_a = dist + 1;
                }

                process_queue.push_back((i, j, dist + 1));
                visited.insert((i, j));
            }
        }
    }

    println!("Day 12 Part 1: {}", shortest_path_to_start);
    println!("Day 12 Part 2: {}", shortest_path_to_a);
}

#[derive(PartialEq, Eq, Debug)]
enum Tree {
    Empty,
    Leaf(u8),
    Tree(Vec<Box<Tree>>),
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Tree::Tree(val), Tree::Tree(other_val)) => val.cmp(&other_val),
            (Tree::Leaf(val), Tree::Leaf(other_val)) => val.cmp(&other_val),
            (Tree::Leaf(val), Tree::Tree(other_val)) => {
                Vec::from([Box::new(Tree::Leaf(*val))]).cmp(&other_val)
            }
            (Tree::Tree(val), Tree::Leaf(other_val)) => {
                val.cmp(&Vec::from([Box::new(Tree::Leaf(*other_val))]))
            }
            (Tree::Empty, Tree::Empty) => Ordering::Equal,
            (Tree::Empty, _) => Ordering::Less,
            (_, Tree::Empty) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Tree {
    fn push(&mut self, val: u8) {
        match self {
            Tree::Tree(children) => children.push(Box::new(Tree::Leaf(val))),
            _ => (),
        }
    }
}

fn build_tree(input: &str) -> Tree {
    // write a little state machine with transitions:
    // NUMERIC -> SEPARATOR: parse int, flush buf
    // NUMERIC -> NUMERIC: just add to buf
    // SEPARATOR -> NUMERIC: just add to buf
    // SEPARATOR -> OPEN [: create new tree, push cur tree to stack, cur tree = new tree
    // OPEN [ -> OPEN [: create new tree, push cur tree to stack, cur tree = new tree
    // NUMERIC -> CLOSE ]: parse int, flush buf, pop old tree from stack, old_tree.push(new_tree), cur tree = old tree
    // CLOSE ] -> CLOSE ]: pop old tree from stack, old_tree.push(new_tree), cur tree = old tree
    // OPEN [ -> NUMERIC: just add to buf

    let mut buf = String::new();
    let mut tree_stack = Vec::<Tree>::new();
    let mut cur_tree = Tree::Empty;
    for c in input.chars() {
        // is_numeric
        if c.is_numeric() {
            buf.push(c);
        } else if c == ',' {
            if !buf.is_empty() {
                let val: u8 = buf.parse().unwrap();
                cur_tree.push(val);
                buf.clear();
            }
        } else if c == '[' {
            tree_stack.push(cur_tree);
            cur_tree = Tree::Tree(Vec::new());
        } else if c == ']' {
            if !buf.is_empty() {
                let val: u8 = buf.parse().unwrap();
                cur_tree.push(val);
                buf.clear();
            }
            if let Tree::Tree(mut old_tree) = tree_stack.pop().unwrap() {
                old_tree.push(Box::new(cur_tree));
                cur_tree = Tree::Tree(old_tree);
            }
        }
    }

    cur_tree
}

fn day_13() {
    let f = File::open("input-13.txt").unwrap();
    let f = BufReader::new(f);

    let mut treebuf: Vec<Tree> = f
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l != "")
        .map(|l| build_tree(&l))
        .collect();

    let total_area_of_ordered_pair_indices: u32 = treebuf
        .chunks(2)
        .zip(1..)
        .map(|(trees, i)| match trees[0].cmp(&trees[1]) {
            Ordering::Less | Ordering::Equal => i,
            _ => 0,
        })
        .sum();

    let divider_1 = build_tree("[[2]]");
    let divider_2 = build_tree("[[6]]");

    treebuf.push(build_tree("[[2]]"));
    treebuf.push(build_tree("[[6]]"));

    treebuf.sort();

    let divider_indices: Vec<usize> = treebuf
        .iter()
        .enumerate()
        .filter(|(i, t)| *t == &divider_1 || *t == &divider_2)
        .map(|(i, _)| i)
        .collect();

    println!("Day 13 Part 1: {}", total_area_of_ordered_pair_indices);
    println!("Day 13 Part 2: {:?}", divider_indices);
}

fn build_walls(f: BufReader<File>) -> HashSet<(u16, u16)> {
    let mut walls: HashSet<(u16, u16)> = HashSet::new();

    walls
}

fn day_14() {
    let f = File::open("input-14.txt").unwrap();
    let f = BufReader::new(f);

    // #1: parse lines to build catcher structure
    // #2: simulate sand falling until one of them reaches the lowest wall built in step #1
    let walls = build_walls(f);

    println!("Day 14 Part 1: {}", 0);
    println!("Day 14 Part 2: {}", 0);
}

fn parse_voxels(f: BufReader<File>) -> HashSet<(i8, i8, i8)> {
    f.lines()
        .map(|l| {
            let l = l.unwrap();
            let mut l = l.split(',');

            (
                l.next().unwrap().parse::<i8>().unwrap(),
                l.next().unwrap().parse::<i8>().unwrap(),
                l.next().unwrap().parse::<i8>().unwrap(),
            )
        })
        .collect()
}

fn day_18() {
    let f = File::open("input-18.txt").unwrap();
    let f = BufReader::new(f);

    let voxels = parse_voxels(f);

    let mut total_area = 0;
    for &v in &voxels {
        if !voxels.contains(&(v.0 + 1, v.1, v.2)) {
            total_area += 1;
        }
        if !voxels.contains(&(v.0 - 1, v.1, v.2)) {
            total_area += 1;
        }
        if !voxels.contains(&(v.0, v.1 + 1, v.2)) {
            total_area += 1;
        }
        if !voxels.contains(&(v.0, v.1 - 1, v.2)) {
            total_area += 1;
        }
        if !voxels.contains(&(v.0, v.1, v.2 + 1)) {
            total_area += 1;
        }
        if !voxels.contains(&(v.0, v.1, v.2 - 1)) {
            total_area += 1;
        }
    }

    let mut visited = HashSet::<(i8, i8, i8)>::new();
    let mut to_visit = VecDeque::<(i8, i8, i8)>::new();
    let mut exterior_area = 0;

    to_visit.push_back((0, 0, 0));

    let max_range = 22;
    let min_range = -1;

    while let Some(v) = to_visit.pop_front() {
        let candidates = [
            (v.0 + 1, v.1, v.2),
            (v.0 - 1, v.1, v.2),
            (v.0, v.1+1, v.2),
            (v.0, v.1-1, v.2),
            (v.0, v.1, v.2+1),
            (v.0, v.1, v.2-1),
        ];
        for c in candidates {
            if c.0 <= max_range && c.0 >= min_range && c.1 <= max_range && c.1 >= min_range && c.2 <= max_range && c.2 >= min_range {
                if voxels.contains(&c) {
                    exterior_area += 1;
                } else if !visited.contains(&c) {
                    to_visit.push_back(c);
                    visited.insert(c);
                }
            }
        }
    }

    println!("Day 18 Part 1: {}", total_area);
    println!("Day 18 Part 2: {}", exterior_area);
}
