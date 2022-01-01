use std::{io::{self, Read}, collections::HashMap, ops::RangeBounds};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Position = [i32; 3];

#[derive(PartialEq, Eq, Hash)]
struct Scanner {
    beacons: Vec<Position>
}

type ShuffleTransform = (usize, usize, usize);
type SignTransform = (bool, bool, bool);
type Transform = (ShuffleTransform, SignTransform);

const SHUFFLES: [ShuffleTransform; 6] = [
    (0, 1, 2), (0, 2, 1),
    (1, 0, 2), (1, 2, 0),
    (2, 0, 1), (2, 1, 0)
];

const SIGNS: [SignTransform; 8] = [
    (true,  true,  true ),
    (true,  true,  false),
    (true,  false, true ),
    (true,  false, false),
    (false, true,  true ),
    (false, true,  false),
    (false, false, true ),
    (false, false, false)
];

const TRANSFORMS: [Transform; 48] = [
    (SHUFFLES[0], SIGNS[0]), (SHUFFLES[0], SIGNS[1]),
    (SHUFFLES[0], SIGNS[2]), (SHUFFLES[0], SIGNS[3]),
    (SHUFFLES[0], SIGNS[4]), (SHUFFLES[0], SIGNS[5]),
    (SHUFFLES[0], SIGNS[6]), (SHUFFLES[0], SIGNS[7]),

    (SHUFFLES[1], SIGNS[0]), (SHUFFLES[1], SIGNS[1]),
    (SHUFFLES[1], SIGNS[2]), (SHUFFLES[1], SIGNS[3]),
    (SHUFFLES[1], SIGNS[4]), (SHUFFLES[1], SIGNS[5]),
    (SHUFFLES[1], SIGNS[6]), (SHUFFLES[1], SIGNS[7]),

    (SHUFFLES[2], SIGNS[0]), (SHUFFLES[2], SIGNS[1]),
    (SHUFFLES[2], SIGNS[2]), (SHUFFLES[2], SIGNS[3]),
    (SHUFFLES[2], SIGNS[4]), (SHUFFLES[2], SIGNS[5]),
    (SHUFFLES[2], SIGNS[6]), (SHUFFLES[2], SIGNS[7]),

    (SHUFFLES[3], SIGNS[0]), (SHUFFLES[3], SIGNS[1]),
    (SHUFFLES[3], SIGNS[2]), (SHUFFLES[3], SIGNS[3]),
    (SHUFFLES[3], SIGNS[4]), (SHUFFLES[3], SIGNS[5]),
    (SHUFFLES[3], SIGNS[6]), (SHUFFLES[3], SIGNS[7]),

    (SHUFFLES[4], SIGNS[0]), (SHUFFLES[4], SIGNS[1]),
    (SHUFFLES[4], SIGNS[2]), (SHUFFLES[4], SIGNS[3]),
    (SHUFFLES[4], SIGNS[4]), (SHUFFLES[4], SIGNS[5]),
    (SHUFFLES[4], SIGNS[6]), (SHUFFLES[4], SIGNS[7]),

    (SHUFFLES[5], SIGNS[0]), (SHUFFLES[5], SIGNS[1]),
    (SHUFFLES[5], SIGNS[2]), (SHUFFLES[5], SIGNS[3]),
    (SHUFFLES[5], SIGNS[4]), (SHUFFLES[5], SIGNS[5]),
    (SHUFFLES[5], SIGNS[6]), (SHUFFLES[5], SIGNS[7]),
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

        out.push(Scanner {
            beacons: scanner
        });
        i += 2;
    }

    out
}

fn apply_transform(p: Position, t: Transform) -> Position {
    [
        p[t.0.0] * if t.1.0 { -1 } else { 1 },
        p[t.0.1] * if t.1.1 { -1 } else { 1 },
        p[t.0.2] * if t.1.2 { -1 } else { 1 }
    ]
}

fn apply_opposite_transform(p: Position, t: Transform) -> Position {
    [
        p[t.0.2] * if t.1.0 { 1 } else { -1 },
        p[t.0.1] * if t.1.1 { 1 } else { -1 },
        p[t.0.0] * if t.1.2 { 1 } else { -1 }
    ]
}

fn get_offset(a: Position, b: Position) -> Position {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Solve for the transform and relative position of input_b to input_a.
fn solve(input_a: &Scanner, input_b: &Scanner) -> Option<(Position, Transform)> {
    for t in TRANSFORMS {
        // apply transform to scanner a's beacons
        let mut new_beacons = Vec::new();
        for beacon in &input_a.beacons {
            new_beacons.push(apply_transform(*beacon, t));
        }

        // check to see if there is a constant offset between at least 12
        // beacons in scanner a's set and scanner b's set
        let mut offsets: HashMap<Position, u32> = HashMap::new();
        for i in new_beacons {
            for j in &input_b.beacons {
                let offset = get_offset(i, *j);
                *offsets.entry(offset).or_insert(0) += 1;
            }
        }

        for (k, v) in offsets.iter() {
            if *v >= 12 {
                // we have 12 matches!
                return Some((*k, t));
            }
        }
    }

    None
}

fn add_pos(a: Position, b: Position) -> Position {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn part1(input: &String) -> EmptyResult {
    let scanners = parse(input);

    let mut beacons: Vec<Position> = Vec::new();
    let mut positions: HashMap<usize, Position> = HashMap::new();
    positions.insert(0, [0, 0, 0]);
    
    for i in 0 .. scanners.len() {
        for j in 0 .. scanners.len() {
            if i != j {
                let solution = solve(&scanners[i], &scanners[j]);
                if let Some((p, t)) = solution {
                    let absolute: Position;
                    let scanner: &Scanner;

                    if let Some(pos) = positions.get(&i) {
                        absolute = add_pos(p, *pos);
                        scanner = &scanners[j];
                        positions.insert(j, absolute);
                    } else if let Some(pos) = positions.get(&j) {
                        absolute = add_pos(p, *pos);
                        scanner = &scanners[i];
                        positions.insert(i, absolute);
                    } else {
                        panic!("fail");
                    }

                    // go through each point in beacon
                    // apply the opposite transformation to it,
                    // and then offset it by the scanner offset
                    // and then map it
                    for beacon in &scanner.beacons {
                        let new_beacon = add_pos(absolute, apply_transform(*beacon, t));
                        if !beacons.contains(&new_beacon) {
                            beacons.push(new_beacon);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", solve(&scanners[0], &scanners[1]));

    println!("{:?}", positions);
    println!("part 1: {}", beacons.len());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
