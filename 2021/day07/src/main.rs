use std::{io::{self, Read}, collections::HashMap};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for strnum in input.strip_suffix('\n').unwrap().split(',') {
        res.push(strnum.parse::<i32>().unwrap());
    }

    res
}

fn min(input: &Vec<i32>) -> i32 {
    let mut min: i32 = i32::MAX;
    for i in input {
        if min > *i {
            min = *i;
        }
    }

    min
}

fn max(input: &Vec<i32>) -> i32 {
    let mut max: i32 = i32::MIN;
    for i in input {
        if max < *i {
            max = *i;
        }
    }

    max
}

fn part1(input: &String) -> EmptyResult {
    let crabs = parse(&input);
    let mut fuel: HashMap<i32, i32> = HashMap::new();

    for target in min(&crabs) .. max(&crabs) {
        let mut fuel2: i32 = 0;
        crabs.iter().for_each(|pos| fuel2 += (pos - target).abs());
        
        fuel.insert(target, fuel2);
    }

    println!("part 1: {}", fuel.values().min().unwrap());
    Ok(())
}

fn move_cost(start: i32, end: i32, cost_results: &mut HashMap<i32, i32>) -> i32 {
    let diff: i32 = (end - start).abs();
    if cost_results.contains_key(&diff) {
        return cost_results[&diff];
    }

    let mut sum: i32 = 0;
    for i in 1 ..= diff {
        sum += i;
    }

    cost_results.insert(diff, sum);

    sum
}

fn part2(input: &String) -> EmptyResult {
    let crabs = parse(&input);
    let mut fuel: HashMap<i32, i32> = HashMap::new();
    let mut costs: HashMap<i32, i32> = HashMap::new();

    for target in min(&crabs) .. max(&crabs) {
        let mut fuel2: i32 = 0;
        crabs.iter().for_each(|pos| fuel2 += move_cost(*pos, target, &mut costs));
        
        fuel.insert(target, fuel2);
    }

    println!("part 2: {}", fuel.values().min().unwrap());
    Ok(())
}
