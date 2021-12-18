use std::io::{self, Read};

const SIGNALS: [[u8; 7]; 10] = [
    [1, 1, 1, 0, 1, 1, 1],
    [0, 0, 1, 0, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 0, 1, 1],
    [0, 1, 1, 1, 0, 1, 0],
    [1, 1, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1]
];

type Numbers = (usize, usize, usize, usize, usize, usize, usize);
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn try_solve(nums: Numbers, input: &str) -> u32 {
    let parts: Vec<&str> = input.split('|').collect();
    let digits: Vec<&str> = parts[0].split_whitespace().collect();
    let result: Vec<&str> = parts[1].split_whitespace().collect();

    for digit in digits {
        let mut segs: Vec<usize> = Vec::new();
        for ch in digit.chars() {
            segs.push(match ch {
                'a' => nums.0,
                'b' => nums.1,
                'c' => nums.2,
                'd' => nums.3,
                'e' => nums.4,
                'f' => nums.5,
                'g' => nums.6,
                _ => panic!("???")
            });
        }

        let mut num: [u8; 7] = [0; 7];
        for seg in segs {
            num[seg] = 1;
        }

        if !SIGNALS.contains(&num) {
            return 0;
        }
    }

    // correct combination
    let mut num = 0;
    for digit in result {
        match digit.len() {
            2 => num += 1,
            3 => num += 1,
            4 => num += 1,
            7 => num += 1,
            _ => ()
        }
    }

    num
}

fn part1(input: &String) -> EmptyResult {
    let mut occurrences: u32 = 0;

    for l in input.lines() {
        let parts: Vec<&str> = l.split('|').collect();
        let result: Vec<&str> = parts.last().unwrap().split_whitespace().collect();

        for word in result {
            match word.len() {
                2 => occurrences += 1,
                3 => occurrences += 1,
                4 => occurrences += 1,
                7 => occurrences += 1,
                _ => ()
            }
        }
    }

    println!("part 1: {}", occurrences);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
