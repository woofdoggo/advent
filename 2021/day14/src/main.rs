use std::{io::{self, Read}, collections::HashMap};

type Pair = [u8; 2];
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[inline(always)]
fn cti(input: char) -> u8 {
    (input as u8) - 'A' as u8
}

fn parse(input: &String) -> (Vec<Pair>, HashMap<Pair, u8>){
    let lines: Vec<&str> = input.lines().collect();
    let seq = String::from(*lines.first().unwrap());
    let mut mapping_vec: Vec<(String, char)> = Vec::new();

    for i in 2 .. lines.len() {
        let mut words = lines[i].split("->");
        let a = words.next().unwrap().to_string().trim().to_string();
        let b = words.last().unwrap().to_string().trim().to_string();
        mapping_vec.push((a, b.chars().next().unwrap()));
    }

    let mut pairs: Vec<Pair> = Vec::new();
    let mut mappings: HashMap<Pair, u8> = HashMap::new();

    for i in 1 .. seq.len() {
        pairs.push([
            cti(seq.chars().nth(i - 1).unwrap()),
            cti(seq.chars().nth(i).unwrap())
        ]);
    }

    for mapping in mapping_vec {
        mappings.insert(
            [
                cti(mapping.0.chars().next().unwrap()),
                cti(mapping.0.chars().last().unwrap())
            ],
            cti(mapping.1)
        );
    }

    (pairs, mappings)
}

fn cycle(pairs: &Vec<Pair>, mappings: &HashMap<Pair, u8>) -> Vec<Pair> {
    let mut result: Vec<Pair> = Vec::new();

    for pair in pairs {
        let middle = mappings.get(pair).unwrap();
        let a: Pair = [pair[0], *middle];
        let b: Pair = [*middle, pair[1]];

        result.push(a);
        result.push(b);
    }

    result
}

fn part1(input: &String) -> EmptyResult {
    let (mut pairs, mappings) = parse(input);

    for _ in 0 .. 10 {
        pairs = cycle(&pairs, &mappings);
    }

    // find most and least common elements
    let mut results: HashMap<u8, u32> = HashMap::new();
    for pair in &pairs {
        *results.entry(pair[0]).or_insert(0) += 1;
    }

    *results.entry(pairs.last().unwrap()[1]).or_insert(0) += 1;

    let result = results.values().max().unwrap() - results.values().min().unwrap();
    println!("part 1: {}", result);

    Ok(())
}

fn pairs_to_bytes(input: &Vec<Pair>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for pair in input {
        output.push(pair[0]);
    }

    output.push(input.last().unwrap()[1]);

    output
}

fn part2(input: &String) -> EmptyResult {
    let (pairs, mappings) = parse(input);
    let mut cache: HashMap<Pair, Vec<u8>> = HashMap::new();
    let mut count: HashMap<Pair, HashMap<u8, u32>> = HashMap::new();

    // generate results/element counts for 20 iterations of each pair
    for pair in mappings.keys() {
        let mut result = vec![*pair];
        for _ in 0 .. 20 {
            result = cycle(&result, &mappings);
        }
        cache.insert(*pair, pairs_to_bytes(&result));
        
        let mut quantity: HashMap<u8, u32> = HashMap::new();
        for pair2 in &result {
            *quantity.entry(pair2[0]).or_insert(0) += 1;
        }
        *quantity.entry(result.last().unwrap()[1]).or_insert(0) += 1;
        count.insert(*pair, quantity);

        println!("cached {:?} with pair count of {}", pair, result.len());
    }

    // simulate using generated results
    let mut results: HashMap<u8, u64> = HashMap::new();
    for pair in pairs {
        let simulated = cache.get(&pair).unwrap();
        for i in 1 .. simulated.len() {
            let current: Pair = [simulated[i - 1], simulated[i]];
            let counts = count.get(&current).unwrap();
            for pair in counts {
                *results.entry(*pair.0).or_insert(0) += *pair.1 as u64;
            }
        }
    }

    let result = results.values().max().unwrap() - results.values().min().unwrap();
    println!("part 2 results: {:?}", results);
    println!("part 2: {}", result);

    Ok(())
}
