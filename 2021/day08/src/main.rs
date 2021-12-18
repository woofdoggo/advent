use std::{io::{self, Read}, ops::Deref};

// me when no Copy trait
const NONE: Option<String> = None;
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

fn sort(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| a.cmp(&b));

    chars.iter().collect()
}

fn try_solve(nums: Numbers, input: &str) -> u32 {
    let parts: Vec<&str> = input.split('|').collect();
    let digits: Vec<&str> = parts[0].split_whitespace().collect();
    let result: Vec<&str> = parts[1].split_whitespace().collect();

    let mut digitmap: [Option<String>; 10] = [NONE; 10];
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

        let index = SIGNALS.iter().position(|&el| el == num);
        match index {
            Some(idx) => digitmap[idx] = Some(sort(digit)),
            None => return 0
        }
    }

    let mut res: u32 = 0;

    for (idx, digit) in result.iter().rev().enumerate() {
        let value = digitmap.iter().position(|d| {
            match d {
                Some(d2) => d2.deref() == sort(digit),
                None => panic!("??? 2")
            }
        }).unwrap() as u32;
        res += value * 10_u32.pow(idx as u32);
    }

    res
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
    let mut sum: u32 = 0;

    for l in input.lines() {
        for a in 0 .. 7 {
            for b in 0 .. 7 {
                if b == a {
                    continue;
                }
                for c in 0 .. 7 {
                    if c == a || c == b {
                        continue;
                    }
                    for d in 0 .. 7 {
                        if d == a || d == b || d == c {
                            continue;
                        }
                        for e in 0 .. 7 {
                            if e == a || e == b || e == c || e == d {
                                continue;
                            }
                            for f in 0 .. 7 {
                                if f == a || f == b || f == c || f == d || f == e {
                                    continue;
                                }

                                for g in 0 .. 7 {
                                    if g == a || g == b || g == c || g == d || g == e || g == f {
                                        continue;
                                    }

                                    sum += try_solve((a, b, c, d, e, f, g), &l);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("part 2: {}", sum);
    Ok(())
}
