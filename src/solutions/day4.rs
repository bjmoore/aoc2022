use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: BufReader<File>) -> Option<(i32, i32)> {
    let mut contained = 0;
    let mut overlap = 0;

    for line in input.lines() {
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

    Some((contained, overlap))
}
