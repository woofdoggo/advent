use std::{io::{self, Read}, collections::HashMap};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> (String, HashMap<String, String>) {
    let lines: Vec<&str> = input.lines().collect();
    let seq = String::from(*lines.first().unwrap());
    let mut mapping: HashMap<String, String> = HashMap::new();

    for i in 2 .. lines.len() {
        let mut words = lines[i].split("->");
        let a = words.next().unwrap().to_string().trim().to_string();
        let b = words.last().unwrap().to_string().trim().to_string();
        mapping.insert(a, b);
    }

    (seq, mapping)
}

fn cycle(polymer: &String, mappings: &HashMap<String, String>) -> String {
    let mut res = String::with_capacity(polymer.capacity() * 3);
    for i in 1 .. polymer.len() {
        res.push(polymer.chars().nth(i - 1).unwrap());
        res.push_str(mappings.get(&polymer[i - 1 .. i + 1]).unwrap());
        res.push(polymer.chars().nth(i).unwrap());
    }

    res
}

fn part1(input: &String) -> EmptyResult {
    let (mut polymer, mapping) = parse(input);

    for _ in 0 .. 10 {
        let n_polymer = cycle(&polymer, &mapping);
        polymer = n_polymer;

        println!("{}", polymer.len());
    }

    println!("part 1: ");
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
