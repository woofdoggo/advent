use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> EmptyResult {
    let mut depth_increases: u32 = 0;
    let mut last_depth: u32 = 0;

    for line in input.lines() {
        let depth = line.parse::<u32>()?;
        if depth > last_depth {
            depth_increases += 1;
        }

        last_depth = depth;
    }

    println!("Depth increases: {}", depth_increases - 1);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let mut depth_increases: u32 = 0;
    let mut last_sum: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();

    for line_num in 3 .. lines.len() + 1 {
        let mut sum: u32 = 0;
        for sum_line in line_num - 3 .. line_num {
            sum += lines[sum_line].parse::<u32>()?;
        }

        if sum > last_sum {
            depth_increases += 1;
        }
        last_sum = sum;
    }

    println!("Depth increases: {}", depth_increases - 1);
    Ok(())
}
