use std::{io::{self, Read}, collections::HashMap};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Position = [i32; 3];
type Scanner = Vec<Position>;

type Transform = [[i32; 3]; 3];
const TRANSFORMS: [Transform; 24] = [
    [[1,0,0],[0,1,0],[0,0,1]],
    [[1,0,0],[0,0,-1],[0,1,0]],
    [[1,0,0],[0,-1,0],[0,0,-1]],
    [[1,0,0],[0,0,-1],[0,-1,0]],
    [[0,-1,0],[1,0,0],[0,0,1]],
    [[0,0,1],[1,0,0],[0,1,0]],
    [[0,1,0],[1,0,0],[0,0,-1]],
    [[0,0,-1],[1,0,0],[0,-1,0]],
    [[-1,0,0],[0,-1,0],[0,0,1]],
    [[-1,0,0],[0,0,-1],[0,-1,0]],
    [[-1,0,0],[0,1,0],[0,0,-1]],
    [[-1,0,0],[0,0,1],[0,1,0]],
    [[0,1,0],[-1,0,0],[0,0,1]],
    [[0,0,1],[-1,0,0],[0,-1,0]],
    [[0,-1,0],[-1,0,0],[0,0,-1]],
    [[0,0,-1],[-1,0,0],[0,1,0]],
    [[0,0,-1],[0,1,0],[1,0,0]],
    [[0,1,0],[0,0,1],[1,0,0]],
    [[0,0,1],[0,-1,0],[1,0,0]],
    [[0,-1,0],[0,0,-1],[1,0,0]],
    [[0,0,-1],[0,-1,0],[-1,0,0]],
    [[0,-1,0],[0,0,1],[-1,0,0]],
    [[0,0,1],[0,1,0],[-1,0,0]],
    [[0,1,0],[0,0,-1],[-1,0,0]]
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

        out.push(scanner);
        i += 2;
    }

    out
}

fn apply_transform(p: Position, t: Transform) -> Position {
    [
        t[0][0] * p[0] + t[0][1] * p[1] + t[0][2] * p[2],
        t[1][0] * p[0] + t[1][1] * p[1] + t[1][2] * p[2],
        t[2][0] * p[0] + t[2][1] * p[1] + t[2][2] * p[2]
    ]
}

fn get_offset(a: Position, b: Position) -> Position {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Solve for the transform and relative position of input_b to input_a.
fn solve(input_a: &Scanner, input_b: &Scanner) -> Option<(Position, Transform)> {
    for t in TRANSFORMS {
        // apply transform to scanner b's beacons
        let mut new_beacons = Vec::new();
        for beacon in input_b {
            new_beacons.push(apply_transform(*beacon, t));
        }

        // check to see if there is a constant offset between at least 12
        // beacons in scanner a's set and scanner b's set
        let mut offsets: HashMap<Position, u32> = HashMap::new();
        for i in new_beacons {
            for j in input_a {
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

struct Solver {
    scanners: Vec<Scanner>,
    beacons: Vec<Position>,
    visited: Vec<usize>
}

impl Solver {
    fn solve(scanners: Vec<Scanner>) -> usize {
        let mut solver = Solver { 
            scanners, 
            beacons: Vec::new(),
            visited: vec![0]
        };

        // add positions from scanner 0
        for beacon in &solver.scanners[0] {
            solver.beacons.push(*beacon);
        }

        // solve
        solver.iterate(0, [0,0,0]);
        return solver.beacons.len();
    }

    fn iterate(&mut self, scanner: usize, position: Position) {
        println!("scanner: {} | pos: {:?}", scanner, position);
        for i in 0 .. self.scanners.len() {
            if !self.visited.contains(&i) {
                let solution = solve(&self.scanners[scanner], &self.scanners[i]);
                if let Some((p, t)) = solution {
                    for beacon in &self.scanners[i] {
                        let new_beacon = apply_transform(*beacon, t);
                        if !self.beacons.contains(&new_beacon) {
                            self.beacons.push(new_beacon)
                        }
                    }

                    // iterate another level
                    self.visited.push(i);
                    println!("iterate to scanner {} | relative: {:?}", i, p);

                    self.iterate(i, add_pos(
                            position,
                            p
                    ));
                }
            }
        }
    }
}

fn part1(input: &String) -> EmptyResult {
    println!("part 1: {}", Solver::solve(parse(input)));
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
