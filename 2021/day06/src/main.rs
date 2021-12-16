use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<u64> {
    let mut res: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    for strnum in input.strip_suffix('\n').unwrap().split(',') {
        let num = strnum.parse::<usize>().unwrap();
        res[num] += 1;
    }

    res
}

fn simulate(fish: Vec<u64>) -> Vec<u64> {
    let mut res: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0 .. 9 {
        match i {
            0 => {
                res[6] = fish[0];
                res[8] = fish[0];
            },
            _ => res[i - 1] += fish[i]
        }
    }

    res
}

fn part1(input: &String) -> EmptyResult {
    let mut fish: Vec<u64> = parse(&input);

    for _ in 0 .. 80 {
        fish = simulate(fish);
    }

    let sum: u64 = fish.iter().sum();
    println!("part 1: {}", sum);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let mut fish = parse(&input);

    for _ in 0 .. 256 {
        fish = simulate(fish);
    }

    let sum: u64 = fish.iter().sum();
    println!("part 2: {}", sum);
    Ok(())
}
