use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn bits_in_column(column: usize, input: &Vec<&str>) -> usize {
    input.iter().map(|l| if l.chars().nth(column) == Some('1') { 1 } else { 0 }).sum()
}

fn part1(input: &String) -> EmptyResult {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..12 {
        let bits = bits_in_column(i, &input.lines().collect());
        if bits >= input.lines().count() - bits {
            gamma |= 1 << 11 - i;
        } else {
            epsilon |= 1 << 11 - i;
        }
    }

    println!("part 1: {}", gamma * epsilon);
    Ok(())
}

fn filter(column: usize, most_common: bool, input: &mut Vec<&str>) {
    let bits = bits_in_column(column, input);
    let threshold = input.len() - bits;

    input.retain(|l| {
        l.chars().nth(column).unwrap() == if most_common {
            if bits >= threshold {
                '1'
            } else {
                '0'
            }
        } else {
            if bits >= threshold {
                '0'
            } else {
                '1'
            }
        }
    });
}

fn part2(input: &String) -> EmptyResult {
    let oxygen;
    let carbon;

    // oxygen
    let mut values: Vec<&str> = input.lines().collect();
    let mut i: usize = 0;
    while values.len() > 1 {
        filter(i, true, &mut values);
        i += 1;
    }
    oxygen = u32::from_str_radix(values.first().unwrap(), 2)?;

    // carbon
    values = input.lines().collect();
    i = 0;
    while values.len() > 1 {
        filter(i, false, &mut values);
        i += 1;
    }
    carbon = u32::from_str_radix(values.first().unwrap(), 2)?;

    println!("part 2: {}", oxygen * carbon);
    Ok(())
}
