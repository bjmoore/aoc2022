fn strategy_one(line: &str) -> i32 {
    match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn strategy_two(line: &str) -> i32 {
    match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

pub fn solve(input: Vec<String>) -> Option<(i32, i32)> {
    let (part1, part2) = input.iter().fold((0, 0), |acc, line| {
        (acc.0 + strategy_one(&line), acc.1 + strategy_two(&line))
    });

    Some((part1, part2))
}
