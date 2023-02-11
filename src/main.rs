use clap::Parser;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

pub mod solutions;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,
}

fn output_result(day: u8, result: Result<(String, String), Box<dyn Error>>) {
    match result {
        Ok((part1, part2)) => {
            println!("Day {day} Part 1: {part1}");
            println!("Day {day} Part 2: {part2}");
        }
        Err(err) => {
            println!("Error running day {day}: {}", err.to_string());
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let start = Instant::now();

    if let Some(day) = cli.day {
        output_result(day, solutions::run_one(day, cli.input));
    } else {
        (1..=25).for_each(|i| output_result(i, solutions::run_one(i, None)));
    }

    println!("Total runtime: {}μs", start.elapsed().as_micros());
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
    let f = File::open("input-8.txt").unwrap();
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

    const max_len: u8 = 99;
    let mut scenic_score = [[1u32; max_len as usize]; max_len as usize];

    let mut vis = HashSet::new();
    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut scenic_score_line = [0u32; 10];
        for j in 0..max_len {
            let height = *map.get(&(i, j)).unwrap();
            if height > vis_threshold {
                vis.insert((i, j));
                vis_threshold = height;
            }
            scenic_score[i as usize][j as usize] *= scenic_score_line[(height - 1) as usize];
            for k in 0..10 {
                if (height - 1) < k {
                    scenic_score_line[k as usize] += 1;
                } else {
                    scenic_score_line[k as usize] = 1;
                }
            }
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut scenic_score_line = [0u32; 10];
        for j in 0..max_len {
            let height = *map.get(&(j, i)).unwrap();
            if height > vis_threshold {
                vis.insert((j, i));
                vis_threshold = height;
            }
            scenic_score[j as usize][i as usize] *= scenic_score_line[(height - 1) as usize];
            for k in 0..10 {
                if (height - 1) < k {
                    scenic_score_line[k as usize] += 1;
                } else {
                    scenic_score_line[k as usize] = 1;
                }
            }
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut scenic_score_line = [0u32; 10];
        for j in (0..max_len).rev() {
            let height = *map.get(&(i, j)).unwrap();
            if height > vis_threshold {
                vis.insert((i, j));
                vis_threshold = height;
            }
            scenic_score[i as usize][j as usize] *= scenic_score_line[(height - 1) as usize];
            for k in 0..10 {
                if (height - 1) < k {
                    scenic_score_line[k as usize] += 1;
                } else {
                    scenic_score_line[k as usize] = 1;
                }
            }
        }
    }

    for i in 0..max_len {
        let mut vis_threshold = 0;
        let mut scenic_score_line = [0u32; 10];
        for j in (0..max_len).rev() {
            let height = *map.get(&(j, i)).unwrap();
            if height > vis_threshold {
                vis.insert((j, i));
                vis_threshold = height;
            }
            scenic_score[j as usize][i as usize] *= scenic_score_line[(height - 1) as usize];
            for k in 0..10 {
                if (height - 1) < k {
                    scenic_score_line[k as usize] += 1;
                } else {
                    scenic_score_line[k as usize] = 1;
                }
            }
        }
    }

    let max_scenic_score = scenic_score.iter().flatten().max().unwrap();

    println!("Day 8 Part 1: {}", vis.len());
    println!("Day 8 Part 2: {:?}", max_scenic_score);
}

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
struct ParseMovementError;

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').ok_or(ParseMovementError)?;

        let dir_fromstr = match dir {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(ParseMovementError),
        }?;
        let dist_fromstr = dist.parse::<i32>().map_err(|_| ParseMovementError)?;

        Ok(Movement {
            direction: dir_fromstr,
            distance: dist_fromstr,
        })
    }
}

fn touching(head_position: &[i32; 2], tail_position: &[i32; 2]) -> bool {
    tail_position[0] > head_position[0] - 2
        && tail_position[0] < head_position[0] + 2
        && tail_position[1] > head_position[1] - 2
        && tail_position[1] < head_position[1] + 2
}

fn day_9() {
    let f = File::open("input-9.txt").unwrap();
    let f = BufReader::new(f);

    let mut tail_positions = HashSet::<[i32; 2]>::new();
    let mut head_position = [0i32, 0];
    let mut tail_position = [0i32, 0];

    tail_positions.insert([0, 0]);

    let movements: Vec<_> = f
        .lines()
        .map(|l| l.unwrap().parse::<Movement>().unwrap())
        .collect();
    let mut full_rope = [[0i32; 2]; 10];
    let mut full_tail_positions = HashSet::<[i32; 2]>::new();

    full_tail_positions.insert([0, 0]);

    for m in &movements {
        // update HEAD position
        match m.direction {
            Direction::U => head_position[0] += m.distance,
            Direction::D => head_position[0] -= m.distance,
            Direction::L => head_position[1] += m.distance,
            Direction::R => head_position[1] -= m.distance,
        }

        // while not_touching(head, tail) {
        //  if tail, head same row or column:
        //      move tail 1 toward head
        //  else:
        //      move tail toward head on each axis
        //  add tail pos to tail_positions
        while !touching(&head_position, &tail_position) {
            if tail_position[0] > head_position[0] {
                tail_position[0] -= 1;
            } else if tail_position[0] < head_position[0] {
                tail_position[0] += 1;
            }
            if tail_position[1] > head_position[1] {
                tail_position[1] -= 1;
            } else if tail_position[1] < head_position[1] {
                tail_position[1] += 1;
            }
            tail_positions.insert(tail_position);
        }
    }

    for m in &movements {
        let mut goal = full_rope[0].clone();
        match m.direction {
            Direction::U => goal[0] += m.distance,
            Direction::D => goal[0] -= m.distance,
            Direction::L => goal[1] += m.distance,
            Direction::R => goal[1] -= m.distance,
        }

        while full_rope[0] != goal {
            if full_rope[0][0] > goal[0] {
                full_rope[0][0] -= 1;
            } else if full_rope[0][0] < goal[0] {
                full_rope[0][0] += 1;
            }
            if full_rope[0][1] > goal[1] {
                full_rope[0][1] -= 1;
            } else if full_rope[0][1] < goal[1] {
                full_rope[0][1] += 1;
            }

            for k in 0..9 {
                while !touching(&full_rope[k], &full_rope[k + 1]) {
                    if full_rope[k + 1][0] > full_rope[k][0] {
                        full_rope[k + 1][0] -= 1;
                    } else if full_rope[k + 1][0] < full_rope[k][0] {
                        full_rope[k + 1][0] += 1;
                    }
                    if full_rope[k + 1][1] > full_rope[k][1] {
                        full_rope[k + 1][1] -= 1;
                    } else if full_rope[k + 1][1] < full_rope[k][1] {
                        full_rope[k + 1][1] += 1;
                    }

                    if k == 8 {
                        full_tail_positions.insert(full_rope[9]);
                    }
                }
            }
        }
    }

    println!("Day 9 Part 1: {}", tail_positions.len());
    println!("Day 9 Part 2: {}", full_tail_positions.len());
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

fn build_walls(f: BufReader<File>) -> HashSet<[u16; 2]> {
    let mut walls: HashSet<[u16; 2]> = HashSet::new();

    for line in f.lines() {
        let line = line.unwrap();
        let points: Vec<[u16; 2]> = line
            .split(" -> ")
            .map(|p| {
                let p = p.split_once(',').unwrap();
                [p.0.parse::<u16>().unwrap(), p.1.parse::<u16>().unwrap()]
            })
            .collect();

        let mut ptr = points[0];
        walls.insert(ptr);
        for &p in points.iter() {
            while ptr != p {
                if ptr[0] < p[0] {
                    ptr[0] += 1
                } else if ptr[0] > p[0] {
                    ptr[0] -= 1
                } else if ptr[1] < p[1] {
                    ptr[1] += 1
                } else if ptr[1] > p[1] {
                    ptr[1] -= 1
                }
                walls.insert(ptr);
            }
        }
    }

    walls
}

struct SandStream {
    source: [u16; 2],
    visit_queue: VecDeque<[u16; 2]>,
}

impl SandStream {
    fn init(
        source: [u16; 2],
        walls: &HashSet<[u16; 2]>,
        sand: &HashSet<[u16; 2]>,
        lowest_floor: u16,
    ) -> Self {
        let mut bottom = source;
        loop {
            if bottom[1] + 1 == lowest_floor
                || walls.contains(&[bottom[0], bottom[1] + 1])
                || sand.contains(&[bottom[0], bottom[1] + 1])
            {
                break;
            } else {
                bottom[1] += 1;
            }
        }
        Self {
            source: source,
            visit_queue: VecDeque::from([bottom]),
        }
    }
}

fn day_14() {
    let f = File::open("input-14.txt").unwrap();
    let f = BufReader::new(f);

    // #1: parse lines to build catcher structure
    let walls = build_walls(f);
    let lowest_floor = walls.iter().map(|w| w[1]).max().unwrap();
    let mut sand = HashSet::<[u16; 2]>::new();

    // #2: simulate sand falling until one of them reaches the lowest wall built in step #1
    'outer: loop {
        let mut sand_ptr = [500u16, 0u16];
        loop {
            if sand_ptr[1] == lowest_floor {
                break 'outer;
            } else if !walls.contains(&[sand_ptr[0], sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0], sand_ptr[1] + 1])
            {
                sand_ptr[1] += 1;
            } else if !walls.contains(&[sand_ptr[0] - 1, sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0] - 1, sand_ptr[1] + 1])
            {
                sand_ptr[0] -= 1;
                sand_ptr[1] += 1;
            } else if !walls.contains(&[sand_ptr[0] + 1, sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0] + 1, sand_ptr[1] + 1])
            {
                sand_ptr[0] += 1;
                sand_ptr[1] += 1;
            } else {
                sand.insert(sand_ptr);
                break;
            }
        }
    }

    let mut sand_2 = HashSet::<[u16; 2]>::new();
    let mut source_stack: Vec<SandStream> = Vec::new();
    // source_stack.push(SandStream::init([500u16, 0]));
    // #3: (lazy..) simulate sand in scenario 2 to find a safe spot
    loop {
        if sand_2.contains(&[500, 0]) {
            break;
        }
        let mut sand_ptr = [500u16, 0u16];
        loop {
            println!("{:?}", sand_ptr);
            if sand_ptr[1] == lowest_floor + 1 {
                sand_2.insert(sand_ptr);
                break;
            } else if !walls.contains(&[sand_ptr[0], sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0], sand_ptr[1] + 1])
            {
                sand_ptr[1] += 1;
            } else if !walls.contains(&[sand_ptr[0] - 1, sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0] - 1, sand_ptr[1] + 1])
            {
                sand_ptr[0] -= 1;
                sand_ptr[1] += 1;
            } else if !walls.contains(&[sand_ptr[0] + 1, sand_ptr[1] + 1])
                && !sand.contains(&[sand_ptr[0] + 1, sand_ptr[1] + 1])
            {
                sand_ptr[0] += 1;
                sand_ptr[1] += 1;
            } else {
                sand_2.insert(sand_ptr);
                break;
            }
        }
    }

    // scan down and build a stream (y value, queue of points to try to populate)
    //  for each point:
    //   if below is open, scan down to new bottom, put a sand there, push old source onto stack and update cur source. new queue.
    //   else if we have a clear path back up to the stream y-axis, insert a sand and any points to visit in the queue
    //   if we build back up to source, pop the source stack and push our cur queue onto the back of the popped queue

    println!("Day 14 Part 1: {}", sand.len());
    println!("Day 14 Part 2: {}", sand_2.len());
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_range: i32,
}

fn parse_sensor_layout(f: BufReader<File>) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        let mut line = line.split(' ');
        // 2 0 4 0
        let bad: &[_] = &['x', 'y', '=', ',', ':'];
        let sensor_x = line
            .nth(2)
            .unwrap()
            .trim_matches(bad)
            .parse::<i32>()
            .unwrap();
        let sensor_y = line
            .nth(0)
            .unwrap()
            .trim_matches(bad)
            .parse::<i32>()
            .unwrap();
        let beacon_x = line
            .nth(4)
            .unwrap()
            .trim_matches(bad)
            .parse::<i32>()
            .unwrap();
        let beacon_y = line
            .nth(0)
            .unwrap()
            .trim_matches(bad)
            .parse::<i32>()
            .unwrap();

        sensors.push(Sensor {
            x: sensor_x,
            y: sensor_y,
            beacon_range: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
        })
    }

    sensors
}

fn merge_intervals(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    intervals.sort_unstable();
    intervals.iter().fold(Vec::new(), |mut acc, &next| {
        if let Some(last) = acc.pop() {
            if next[0] > last[1] {
                acc.push(last);
                acc.push(next);
            } else if next[1] <= last[1] {
                acc.push(last);
            } else {
                acc.push([last[0], next[1]]);
            }
        } else {
            acc.push(next);
        }

        acc
    })
}

fn day_15() {
    let f = File::open("input-15.txt").unwrap();
    let f = BufReader::new(f);

    let sensors = parse_sensor_layout(f);

    let min_x = sensors.iter().map(|s| s.x - s.beacon_range).min().unwrap();
    let max_x = sensors.iter().map(|s| s.x + s.beacon_range).max().unwrap();

    let y = 2000000;
    let y_intervals_2m: Vec<[i32; 2]> = sensors
        .iter()
        .filter_map(|s| {
            let displacement = (s.y - y).abs();
            if displacement <= s.beacon_range {
                Some([
                    s.x - s.beacon_range + displacement,
                    s.x + s.beacon_range - displacement + 1,
                ])
            } else {
                None
            }
        })
        .collect();
    let row_2000000 = merge_intervals(y_intervals_2m);
    let excluded_squares = row_2000000[0][1] - row_2000000[0][0] - 1;

    let (x, y): (u64, u64) = (0..4000000)
        .find_map(|y| {
            let y_intervals: Vec<[i32; 2]> = sensors
                .iter()
                .filter_map(|s| {
                    let displacement = (s.y - y).abs();
                    if displacement <= s.beacon_range {
                        Some([
                            s.x - s.beacon_range + displacement,
                            s.x + s.beacon_range - displacement + 1,
                        ])
                    } else {
                        None
                    }
                })
                .collect();
            let merged = merge_intervals(y_intervals);
            if merged.len() == 2 {
                Some((merged[0][1] as u64, y as u64))
            } else {
                None
            }
        })
        .unwrap();

    println!("Day 15 Part 1: {}", excluded_squares);
    println!("Day 15 Part 2: {}", x * 4000000 + y);
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
            (v.0, v.1 + 1, v.2),
            (v.0, v.1 - 1, v.2),
            (v.0, v.1, v.2 + 1),
            (v.0, v.1, v.2 - 1),
        ];
        for c in candidates {
            if c.0 <= max_range
                && c.0 >= min_range
                && c.1 <= max_range
                && c.1 >= min_range
                && c.2 <= max_range
                && c.2 >= min_range
            {
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
