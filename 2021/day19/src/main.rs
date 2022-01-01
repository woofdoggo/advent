use std::{io::{self, Read}, collections::HashMap};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Position = [i32; 3];
type Scanner = Vec<Position>;

type Transform = [[i32; 3]; 3];
const TRANSFORMS: [Transform; 24] = [
    [[1,0,0],[0,1,0],[0,0,1]],
    [[1,0,0],[0,0,-1],[0,1,0]],
    [[1,0,0],[0,-1,0],[0,0,-1]],
    [[1,0,0],[0,0,-1],[0,-1,0]],
    [[0,-1,0],[1,0,0],[0,0,1]],
    [[0,0,1],[1,0,0],[0,1,0]],
    [[0,1,0],[1,0,0],[0,0,-1]],
    [[0,0,-1],[1,0,0],[0,-1,0]],
    [[-1,0,0],[0,-1,0],[0,0,1]],
    [[-1,0,0],[0,0,-1],[0,-1,0]],
    [[-1,0,0],[0,1,0],[0,0,-1]],
    [[-1,0,0],[0,0,1],[0,1,0]],
    [[0,1,0],[-1,0,0],[0,0,1]],
    [[0,0,1],[-1,0,0],[0,-1,0]],
    [[0,-1,0],[-1,0,0],[0,0,-1]],
    [[0,0,-1],[-1,0,0],[0,1,0]],
    [[0,0,-1],[0,1,0],[1,0,0]],
    [[0,1,0],[0,0,1],[1,0,0]],
    [[0,0,1],[0,-1,0],[1,0,0]],
    [[0,-1,0],[0,0,-1],[1,0,0]],
    [[0,0,-1],[0,-1,0],[-1,0,0]],
    [[0,-1,0],[0,0,1],[-1,0,0]],
    [[0,0,1],[0,1,0],[-1,0,0]],
    [[0,1,0],[0,0,-1],[-1,0,0]]
];

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<Scanner> {
    let lines: Vec<&str> = input.lines().collect();
    let mut out = Vec::new();
    let mut i = 1;

    while i < lines.len() - 1 {
        let mut line = lines[i];
        let mut scanner = Vec::new();

        while !line.is_empty() {
            let mut splits = line.split(',');
            scanner.push([
                splits.next().unwrap().parse::<i32>().unwrap(),
                splits.next().unwrap().parse::<i32>().unwrap(),
                splits.next().unwrap().parse::<i32>().unwrap()
            ]);

            i += 1;
            line = lines[i];
        }

        out.push(scanner);
        i += 2;
    }

    out
}

fn part1(input: &String) -> EmptyResult {

    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
