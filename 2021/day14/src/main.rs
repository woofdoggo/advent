use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> (String, Vec<(String, char)>) {
    let lines: Vec<&str> = input.lines().collect();
    let seq = String::from(*lines.first().unwrap());
    let mut mapping: Vec<(String, char)> = Vec::new();

    for i in 2 .. lines.len() {
        let mut words = lines[i].split("->");
        let a = words.next().unwrap().to_string().trim().to_string();
        let b = words.last().unwrap().to_string().trim().to_string();
        mapping.push((a, b.chars().next().unwrap()));
    }

    (seq, mapping)
}

fn part1(input: &String) -> EmptyResult {
    let (mut polymer, mapping) = parse(input);

    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
