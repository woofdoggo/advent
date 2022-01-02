use std::collections::HashSet;
use std::io::{self, Read};
use std::cmp::{max as max, Ordering};
use std::cmp::min as min;

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

type Cuboid = (Position, Position);
type Position = (i32, i32, i32);
type Reboot = (Cuboid, bool);

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Vec<Reboot> {
    let mut out = Vec::new();
    
    for line in input.lines() {
        let (state, cube) = line.split_once(' ').unwrap();
        let pos: Vec<&str> = cube.split(',').collect();
        let x: Vec<&str> = pos[0].strip_prefix("x=").unwrap().split("..").collect();
        let y: Vec<&str> = pos[1].strip_prefix("y=").unwrap().split("..").collect();
        let z: Vec<&str> = pos[2].strip_prefix("z=").unwrap().split("..").collect();

        out.push((
            (
                (
                    x[0].parse::<i32>().unwrap(),
                    y[0].parse::<i32>().unwrap(),
                    z[0].parse::<i32>().unwrap()
                ),
                (
                    x[1].parse::<i32>().unwrap(),
                    y[1].parse::<i32>().unwrap(),
                    z[1].parse::<i32>().unwrap()
                )
            ),
            match state {
                "on" => true,
                "off" => false,
                _ => panic!("invalid reboot state")
            }
        ));
    }

    out
}

fn intersection(a: Cuboid, b: Cuboid) -> Option<Cuboid> {
    if  a.0.0 <= b.1.0 && a.1.0 >= b.0.0 &&
        a.0.1 <= b.1.1 && a.1.1 >= b.0.1 &&
        a.0.2 <= b.1.2 && a.1.2 >= b.0.2
    {
        Some((
            (
                max(a.0.0, b.0.0),
                max(a.0.1, b.0.1),
                max(a.0.2, b.0.2)
            ),
            (
                min(a.1.0, b.1.0),
                min(a.1.1, b.1.1),
                min(a.1.2, b.1.2)
            )
        ))
    } else {
        None
    }
}

fn cuboid_volume(input: Cuboid) -> u64 {
    ((input.0.0 - input.1.0).abs() as u64 + 1) *
    ((input.0.1 - input.1.1).abs() as u64 + 1) *
    ((input.0.2 - input.1.2).abs() as u64 + 1)
}

fn part1(input: &String) -> EmptyResult {
    // get list of reboots and sort them
    // so that "on" reboots are first.
    let mut reboots = parse(input);
    reboots.sort_by(|a, b| {
        if a.1 == b.1 { Ordering::Equal }
        else if a.1 && !b.1 { Ordering::Greater }
        else { Ordering::Less }
    });

    const VALID_CUBE: Cuboid = ((-50, -50, -50), (50, 50, 50));
    let mut cuboids: Vec<(Cuboid, bool)> = Vec::new();

    for reboot in reboots {
        if let Some(c) = intersection(reboot.0, VALID_CUBE) {
            // check for any intersections with other "ON" cuboids
            let mut sub_intersect = Vec::new();
            for cuboid in &cuboids {
                if let Some(c2) = intersection(c, cuboid.0) {
                    sub_intersect.push((c2, reboot.1 != cuboid.1));
                }
            }

            cuboids.append(&mut sub_intersect);

            // add this cuboid to list of cuboids
            // ONLY if it is on.
            if reboot.1 { cuboids.push(reboot); }
        }
    }

    // calculate cuboid volumes and add them
    let mut sum: u64 = 0;
    for cuboid in &cuboids {
        if cuboid.1 { sum += cuboid_volume(cuboid.0); }
        else { sum -= cuboid_volume(cuboid.0); }
    }

    println!("part 1: {}", sum);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
