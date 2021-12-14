use std::{io::{self, Read}, ops::Deref};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> EmptyResult {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let movement: i32 = words.last().expect("no last word").parse::<i32>()?;
        
        match words.first().expect("no first word").deref() {
            "forward" => pos += movement,
            "down" => depth += movement,
            "up" => depth -= movement,
            _ => println!("unknown match")
        }
    }

    println!("Product of depth/position: {}", pos * depth);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let mut aim: i32 = 0;
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let movement: i32 = words.last().expect("no last word").parse::<i32>()?;
        
        match words.first().expect("no first word").deref() {
            "forward" => {
                pos += movement;
                depth += aim * movement
            },
            "down" => aim += movement,
            "up" => aim -= movement,
            _ => println!("unknown match")
        }
    }

    println!("Product of depth/position: {}", pos * depth);
    Ok(())
}
