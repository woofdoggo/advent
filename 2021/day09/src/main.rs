use std::io::{self, Read};

type PosVec = Vec<(usize, usize)>;
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn get_cell(input: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32> {
    if y > input[0].len() - 1 {
        None
    } else if x > input.len() - 1 {
        None
    } else {
        Some(input[x][y])
    }
}

fn is_low(input: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let a = get_cell(input, x + 1, y);
    let b = if x != 0 { get_cell(input, x - 1, y) } else { None };
    let c = get_cell(input, x, y + 1);
    let d = if y != 0 { get_cell(input, x, y - 1) } else { None };

    let value = input[x][y];

    if a.is_some() && a <= Some(value) { false }
    else if b.is_some() && b <= Some(value) { false }
    else if c.is_some() && c <= Some(value) { false }
    else if d.is_some() && d <= Some(value) { false }
    else { true }
}

fn parse(input: &String) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        res.push(
            line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        );
    }

    res
}

fn part1(input: &String) -> EmptyResult {
    let map = parse(input);
    let mut sum = 0;

    for (i, x) in map.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            if is_low(&map, i, j) {
                sum += map[i][j] + 1;
            }
        }
    }

    println!("part 1: {}", sum);
    Ok(())
}

fn flood_fill(input: &Vec<Vec<u32>>, visits: &mut PosVec, x: usize, y: usize) {
    if visits.contains(&(x, y)) { return; }
    
    match get_cell(input, x, y) {
        Some(cell) => if cell == 9 { return; },
        None => return
    };

    visits.push((x, y));

    if x != 0 {
        flood_fill(&input, visits, x - 1, y);
    }

    if y != 0 {
        flood_fill(&input, visits, x, y - 1 );
    }

    flood_fill(&input, visits, x + 1, y);
    flood_fill(&input, visits, x, y + 1);
}

fn part2(input: &String) -> EmptyResult {
    let map = parse(input);
    let mut basins: Vec<usize> = Vec::new();
    
    for (i, x) in map.iter().enumerate() {
        for (j, _) in x.iter().enumerate() {
            if is_low(&map, i, j) {
                let mut visits: PosVec = Vec::new();

                // iterate over all
                flood_fill(&map, &mut visits, i, j);

                basins.push(visits.len());
            }
        }
    }
    
    basins.sort();
    let l = basins.len();

    println!("part 2: {}", basins[l - 1] * basins[l - 2] * basins[l - 3]);
    Ok(())
}
