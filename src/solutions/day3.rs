use itertools::Itertools;

fn char_bitmask(c: char) -> u64 {
    match c {
        'a' => 1,
        'b' => 1 << 1,
        'c' => 1 << 2,
        'd' => 1 << 3,
        'e' => 1 << 4,
        'f' => 1 << 5,
        'g' => 1 << 6,
        'h' => 1 << 7,
        'i' => 1 << 8,
        'j' => 1 << 9,
        'k' => 1 << 10,
        'l' => 1 << 11,
        'm' => 1 << 12,
        'n' => 1 << 13,
        'o' => 1 << 14,
        'p' => 1 << 15,
        'q' => 1 << 16,
        'r' => 1 << 17,
        's' => 1 << 18,
        't' => 1 << 19,
        'u' => 1 << 20,
        'v' => 1 << 21,
        'w' => 1 << 22,
        'x' => 1 << 23,
        'y' => 1 << 24,
        'z' => 1 << 25,
        'A' => 1 << 26,
        'B' => 1 << 27,
        'C' => 1 << 28,
        'D' => 1 << 29,
        'E' => 1 << 30,
        'F' => 1 << 31,
        'G' => 1 << 32,
        'H' => 1 << 33,
        'I' => 1 << 34,
        'J' => 1 << 35,
        'K' => 1 << 36,
        'L' => 1 << 37,
        'M' => 1 << 38,
        'N' => 1 << 39,
        'O' => 1 << 40,
        'P' => 1 << 41,
        'Q' => 1 << 42,
        'R' => 1 << 43,
        'S' => 1 << 44,
        'T' => 1 << 45,
        'U' => 1 << 46,
        'V' => 1 << 47,
        'W' => 1 << 48,
        'X' => 1 << 49,
        'Y' => 1 << 50,
        'Z' => 1 << 51,
        _ => 0,
    }
}

fn bitwise_log2(mut int: u64) -> i32 {
    let mut count = 0;
    while int > 0 {
        int >>= 1;
        count += 1;
    }
    count
}

pub fn solve(input: Vec<String>) -> Option<(i32, i32)> {
    let part1 = input
        .iter()
        .map(|line| {
            let first_half_bitmask = line
                .chars()
                .take(line.len() / 2)
                .fold(0u64, |acc, next| acc | char_bitmask(next));
            let second_half_bitmask = line
                .chars()
                .skip(line.len() / 2)
                .fold(0u64, |acc, next| acc | char_bitmask(next));
            bitwise_log2(first_half_bitmask & second_half_bitmask)
        })
        .sum();

    let part2 = input
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let bitmask = chunk
                .map(|line| {
                    line.chars()
                        .fold(0u64, |acc, next| acc | char_bitmask(next))
                })
                .reduce(|acc, next| acc & next)
                .unwrap_or(0);
            bitwise_log2(bitmask)
        })
        .sum();

    Some((part1, part2))
}
