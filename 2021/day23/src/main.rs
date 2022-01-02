use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Hallway = [Amphipod; 15];
type Room = [Amphipod; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    hallway: Hallway,
    a: Room,
    b: Room,
    c: Room,
    d: Room
}

#[allow(unused)]
const NOTE: &str = "
- generate all permutations of steps which will 
";

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn cta(input: char) -> Amphipod {
    match input {
        'A' => Amphipod::Amber,
        'B' => Amphipod::Bronze,
        'C' => Amphipod::Copper,
        'D' => Amphipod::Desert,
        _ => panic!("invalid amphipod")
    }
}

fn parse(input: &String) -> State {
    let row_a = input.lines().nth(2).unwrap();
    let row_b = input.lines().nth(3).unwrap();

    State {
        hallway: [Amphipod::Empty; 15],
        a: [cta(row_a.chars().nth(3).unwrap()), cta(row_b.chars().nth(3).unwrap())],
        b: [cta(row_a.chars().nth(5).unwrap()), cta(row_b.chars().nth(5).unwrap())],
        c: [cta(row_a.chars().nth(7).unwrap()), cta(row_b.chars().nth(7).unwrap())],
        d: [cta(row_a.chars().nth(9).unwrap()), cta(row_b.chars().nth(9).unwrap())]
    }
}

fn part1(input: &String) -> EmptyResult {
    println!("{:?}", parse(input));

    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
