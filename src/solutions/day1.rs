use std::error::Error;

pub fn solve(input: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    let mut elfs = vec![0; 3];
    let mut current_elf = 0;

    for line in input.iter() {
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

    Ok((elfs[2].to_string(), elfs.iter().sum::<i32>().to_string()))
}
