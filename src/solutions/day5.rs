use std::error::Error;

use itertools::Itertools;

fn parse_move(input: &String) -> (usize, usize, usize) {
    let (count, from, to) = input
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .take(3)
        .collect_tuple()
        .unwrap();

    (count, from - 1, to - 1)
}

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    // walk through input to find pile starting with " 1"
    let split_index = input
        .iter()
        .position(|s| s.starts_with(" 1"))
        .ok_or("Invalid input, missing line with \" 1\"")?;

    // get number of stacks
    let stack_count = input.get(split_index).unwrap().len() / 4 + 1;
    let mut pile_one: Vec<Vec<char>> = vec![Vec::new(); stack_count];
    let mut pile_two: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    // step backwards through input from there to build 2 piles
    for row in (0..split_index).rev() {
        let boxes = input[row]
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic());
        for (col, label) in boxes {
            pile_one[col].push(label);
            pile_two[col].push(label);
        }
    }

    // apply operations in order to both
    for row in input.iter().skip(split_index + 2) {
        let (count, from, to) = parse_move(row);

        for _ in 0..count {
            let x = pile_one[from].pop().unwrap();
            pile_one[to].push(x);
        }

        let split_at = pile_two[from].len() - count;
        let mut y = pile_two[from].split_off(split_at);
        pile_two[to].append(&mut y);
    }

    let tops_one = pile_one.iter().filter_map(|v| v.last()).collect::<String>();
    let tops_two = pile_two.iter().filter_map(|v| v.last()).collect::<String>();

    Ok((tops_one, tops_two))
}
