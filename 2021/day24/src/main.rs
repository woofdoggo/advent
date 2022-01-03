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

fn run_block(mut z: i64, w: i64, idiff: [i64; 3]) -> i64 {
    let mut x = z;
    x %= 26;
    z /= idiff[0];
    x += idiff[1];
    x = if w == x { 0 } else { 1 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    y = w + idiff[2];
    y *= x;
    z + y
}

fn sim(input: &Vec<i64>, bad: &mut HashSet<i64>) -> bool {
    let mut z = 0;

    for i in 0 .. 14 {
        z = run_block(z, input[i], DIFFS[i]);
        if bad.contains(&z) { return false; }
    }

    if z != 0 { bad.insert(z); return false; }
    return true;
}

fn part1() -> EmptyResult {
    // thank you stackoverflow i didnt want to to_str->split
    // https://stackoverflow.com/questions/41536479/how-do-i-split-an-integer-into-individual-digits
    fn x(n: i64, xs: &mut Vec<i64>) {
        if n >= 10 {
            x(n / 10, xs);
        }
        if n % 10 == 0 { return; }
        xs.push(n % 10);
    }

    let mut bad = HashSet::new();
    for i in (0 ..= 99999999999999_i64).rev() {
        let mut n = Vec::new();
        x(i, &mut n);

        if n.len() < 14 { continue; }
        if sim(&n, &mut bad) {
            println!("part 1: {}", i);
            break;
        }

        if i % 1111111 == 0 { println!("{}", i); }
    }

    Ok(())
}

fn part2() -> EmptyResult {

    Ok(())
}
