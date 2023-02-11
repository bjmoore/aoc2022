use std::error::Error;

use regex::Regex;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut total_area_under_100000 = 0;
    let mut stack = Vec::new();
    let mut dir_sizes = Vec::new();
    let mut current_size: u32 = 0;
    let mut total_used: u32 = 0;

    let filesize_regex = Regex::new(r"^(\d+)").unwrap();

    for line in input {
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
    let smallest_dir_that_makes_space = dir_sizes
        .iter()
        .filter(|x| *x > &space_needed)
        .min()
        .unwrap();

    Ok((
        space_needed.to_string(),
        smallest_dir_that_makes_space.to_string(),
    ))
}
