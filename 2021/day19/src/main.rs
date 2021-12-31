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

fn get_offset(a: Position, b: Position) -> (i32, i32, i32) {
    (a[0] - b[0], a[1] - b[1], a[2] - b[2])
}

/// Solve for the positions of all beacons
/// relative to scanner 0
fn solve(input: &Vec<Scanner>) -> Vec<Position> {
    for ((idx_a, idx_b), dist) in &input[0].distances {
        for ((idx_c, idx_d), dist_2) in &input[1].distances {
            if dist == dist_2 {
                // two pairs of beacons with matching distances.
                // (from different scanners, they may be the same
                // actual physical pair)
                //
                // calculate offset of scanner b -> scanner a for
                // these beacons to have matching positions.
                // (if one exists - they may not actually be the same
                // pair, and so no such matching position would exist)
                //
                // check if positions of at least 12 beacons now match
                // if so, we have found the position of scanner b in
                // relation to scanner a.
                
                let ba = input[0].beacons[*idx_a];
                let bb = input[0].beacons[*idx_b];
                let bc = input[1].beacons[*idx_c];
                let bd = input[1].beacons[*idx_d];

                // we need to match the positions of one of the following:
                // ba->bc && bb->bd
                // ba->bd && bb->bc
                //
                // however, we also have to account for all of the possible
                // transformations which could take place on the
                // rotation/facing of scanner b.

                for t in TRANSFORMS {
                    // transform the positions of the "matching" beacons
                    // that are seen by scanner 0 to match what may
                    // potentially be scanner 1's view
                    let ta = apply_transform(ba, t);
                    let tb = apply_transform(bb, t);

                    // check if there is an equal offset between positions
                    let diff = if get_offset(ta, bc) == get_offset(tb, bd) {
                        Some(get_offset(ta, bc))
                    } else if get_offset(ta, bd) == get_offset(tb, bc) {
                        Some(get_offset(ta, bd))
                    } else {
                        None
                    };

                    if let Some(offset) = diff {
                        println!("wo");
                        // equal offset between pairs!
                        // let's try to apply it to the beacons seen by each scanner
                        // and check that there are a minimum of 12 matches.

                        // apply offset
                        let mut new_beacons: Vec<Position> = Vec::new();
                        for beacon in &input[0].beacons {
                            new_beacons.push([
                                beacon[0] + offset.0,
                                beacon[1] + offset.1,
                                beacon[2] + offset.2
                            ]);
                        }

                        let mut matches = 0;
                        for beacon in new_beacons {
                            for beacon_2 in &input[1].beacons {
                                if  beacon[0] == beacon_2[0] && 
                                    beacon[1] == beacon_2[1] &&
                                    beacon[2] == beacon_2[2] 
                                {
                                    matches += 1;
                                    break
                                }
                            }
                        }

                        if matches != 0 {
                            println!("wo");
                        }
                    }
                }
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
