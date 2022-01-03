use std::{ops::Deref, collections::HashSet};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

const DIFFS: [[i64; 3]; 14] = [
    [1, 14, 16],
    [1, 11, 3],
    [1, 12, 2],
    [1, 11, 7],
    [26, -10, 13],
    [1, 15, 6],
    [26, -14, 10],
    [1, 10, 11],
    [26, -4, 6],
    [26, -3, 5],
    [1, 13, 11],
    [26, -3, 4],
    [26, -9, 4],
    [26, -12, 6]
];

fn main() -> EmptyResult {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> EmptyResult {

    Ok(())
}

fn part2() -> EmptyResult {

    Ok(())
}
