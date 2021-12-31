use std::{io::{self, Read}, collections::HashMap};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Position = [i32; 3];

struct Scanner {
    beacons: Vec<Position>,
    distances: HashMap<(usize, usize), u32>
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
            beacons: scanner,
            distances: HashMap::new()
        });
        i += 2;
    }

    out
}

fn calc_distances(input: &mut Scanner) {
    for i in 0 .. input.beacons.len() {
        for j in 0 .. input.beacons.len() {
            if i != j {
                input.distances.insert((i, j), distance(input.beacons[i], input.beacons[j]));
            }
        }
    }
}

fn apply_transform(p: Position, t: Transform) -> Position {
    [
        p[t.0.0] * if t.1.0 { 1 } else { -1 },
        p[t.0.1] * if t.1.1 { 1 } else { -1 },
        p[t.0.2] * if t.1.2 { 1 } else { -1 },
    ]
}

fn diff(a: i32, b: i32) -> u32 {
    if a > b {
        (a - b) as u32
    } else {
        (b - a) as u32
    }
}

fn distance(a: Position, b: Position) -> u32 {
    diff(a[0], b[0]) + diff(a[1], b[1]) + diff(a[2], b[2])
}

fn get_offset(a: Position, b: Position) -> Position {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Solve for the positions of all beacons
/// relative to scanner 0
fn solve(input: &Vec<Scanner>) -> Vec<Position> {
    for t in TRANSFORMS {
        // apply transform to scanner a's beacons
        let mut new_beacons = Vec::new();
        for beacon in &input[0].beacons {
            new_beacons.push(apply_transform(*beacon, t));
        }

        // check to see if there is a constant offset between at least 12
        // beacons in scanner a's set and scanner b's set
        let mut offsets: HashMap<Position, u32> = HashMap::new();
        for i in new_beacons {
            for j in &input[1].beacons {
                let offset = get_offset(i, *j);
                *offsets.entry(offset).or_insert(0) += 1;
            }
        }

        for val in offsets.values() {
            if *val >= 12 {
                println!("WE DID IT");
            }
        }
    }

    Vec::new()
}

fn part1(input: &String) -> EmptyResult {
    // parse and prepare scanners
    // distances do not change, even with transformation
    let mut scanners = parse(input);
    for mut scanner in &mut scanners {
        calc_distances(&mut scanner);
    }

    println!("part 1: {}", solve(&scanners).len());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
