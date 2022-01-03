use std::{io::{self, Read}, collections::HashSet};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SC {
    Side,
    Down,
    None
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<Vec<SC>> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(match char {
                '>' => SC::Side,
                'v' => SC::Down,
                '.' => SC::None,
                _ => panic!("invalid char")
            });
        }

        out.push(row);
    }

    out
}

fn cycle(seafloor: &mut Vec<Vec<SC>>) -> u32 {
    let mut moves = 0;

    let mut changes: HashSet<(usize, usize, SC)> = HashSet::new();
    for i in 0 .. seafloor.len() {
        for (j, c) in seafloor[i].iter().enumerate() {
            match c {
                SC::Side => {
                    if seafloor[i][(j + 1) % seafloor[i].len()] == SC::None {
                        changes.insert((i, j, SC::None));
                        changes.insert((i, (j + 1) % seafloor[i].len(), SC::Side));
                    }
                },
                _ => continue
            }
        }
    }

    for (x, y, s) in &changes {
        seafloor[*x][*y] = *s;
        moves += 1;
    }

    changes.clear();

    for i in 0 .. seafloor.len() {
        for (j, c) in seafloor[i].iter().enumerate() {
            match c {
                SC::Down => {
                    if seafloor[(i + 1) % seafloor.len()][j] == SC::None {
                        changes.insert((i, j, SC::None));
                        changes.insert(((i + 1) % seafloor.len(), j, SC::Down));
                    }
                },
                _ => continue
            }
        }
    }

    for (x, y, s) in &changes {
        seafloor[*x][*y] = *s;
        moves += 1;
    }

    changes.clear();

    moves
}

fn part1(input: &String) -> EmptyResult {
    let mut seafloor = parse(input);
    let mut steps = 1;
    while cycle(&mut seafloor) != 0 {
        steps += 1;
    }

    println!("part 1: {}", steps);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
