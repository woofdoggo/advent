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

                                    occurrences += try_solve((a, b, c, d, e, f, g), &l);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("part 1: {}", occurrences);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
