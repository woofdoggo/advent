use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(PartialEq, Eq)]
enum BracketType {
    Flat,
    Paren,
    Curly,
    Angle
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> EmptyResult {
    let mut sum = 0;
    for line in input.lines() {
        let mut stack: Vec<BracketType> = Vec::new();
        for char in line.chars() {
            match char {
                '[' => stack.push(BracketType::Flat),
                ']' => {
                    if *stack.last().unwrap() == BracketType::Flat {
                        stack.pop();
                    } else {
                        sum += 57;
                        break;
                    }
                },
                '(' => stack.push(BracketType::Paren),
                ')' => {
                    if *stack.last().unwrap() == BracketType::Paren {
                        stack.pop();
                    } else {
                        sum += 3;
                        break;
                    }
                },
                '{' => stack.push(BracketType::Curly),
                '}' => {
                    if *stack.last().unwrap() == BracketType::Curly {
                        stack.pop();
                    } else {
                        sum += 1197;
                        break;
                    }
                },
                '<' => stack.push(BracketType::Angle),
                '>' => {
                    if *stack.last().unwrap() == BracketType::Angle {
                        stack.pop();
                    } else {
                        sum += 25137;
                        break;
                    }
                },
                _ => panic!("???")
            }
        }
    }

    println!("part 1: {}", sum);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let mut scores: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut stack: Vec<BracketType> = Vec::new();
        let mut good = true;
        for char in line.chars() {
            match char {
                '[' => stack.push(BracketType::Flat),
                ']' => {
                    if *stack.last().unwrap() == BracketType::Flat {
                        stack.pop();
                    } else {
                        good = false;
                        break;
                    }
                },
                '(' => stack.push(BracketType::Paren),
                ')' => {
                    if *stack.last().unwrap() == BracketType::Paren {
                        stack.pop();
                    } else {
                        good = false;
                        break;
                    }
                },
                '{' => stack.push(BracketType::Curly),
                '}' => {
                    if *stack.last().unwrap() == BracketType::Curly {
                        stack.pop();
                    } else {
                        good = false;
                        break;
                    }
                },
                '<' => stack.push(BracketType::Angle),
                '>' => {
                    if *stack.last().unwrap() == BracketType::Angle {
                        stack.pop();
                    } else {
                        good = false;
                        break;
                    }
                },
                _ => panic!("???")
            }
        }

        if good {
            let mut score: u64 = 0;
            while stack.len() > 0 {
                let el = stack.pop().unwrap();

                score *= 5;
                match el {
                    BracketType::Flat => score += 2,
                    BracketType::Paren => score += 1,
                    BracketType::Curly => score += 3,
                    BracketType::Angle => score += 4,
                }
            }

            scores.push(score);
        }
    }

    scores.sort();
    println!("part 2: {}", scores[scores.len() / 2]);
    Ok(())
}
