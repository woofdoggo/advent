mod test;

use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Element {
    Open,
    Close,
    Num(u8)
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &str) -> Vec<Element> {
    let mut result = Vec::new();

    for char in input.chars() {
        match char {
            '[' => result.push(Element::Open),
            ']' => result.push(Element::Close),
            ',' => (),
            _ => result.push(Element::Num(char.to_digit(10).unwrap() as u8))
        }
    }

    result
}

fn add_direction(input: &mut Vec<Element>, mut pos: usize, go_right: bool, add: u8) {
    while pos != 0 && pos != input.len() - 1 {
        if go_right {
            pos += 1;
        } else {
            pos -= 1;
        }

        if let Element::Num(n) = input[pos] {
            input[pos] = Element::Num(n + add);
            return;
        }
    }
}

fn reduce(input: &mut Vec<Element>) {
    let mut on_explode = false;
    let mut consecutive_fails = 0;

    loop {
        let mut pos = 0;
        let mut depth = 0;

        on_explode = !on_explode;
        while pos < input.len() {
            let el = input[pos];
            pos += 1;

            match el {
                // depth change
                Element::Open => depth += 1,
                Element::Close => depth -= 1,

                // number
                Element::Num(num) => {
                    if on_explode {
                        if depth > 4 {
                            // 4 deep - check for explosion!
                            if let Element::Num(n) = input[pos] {
                                // we are in a pair. explode
                                add_direction(input, pos - 1, false, num);
                                add_direction(input, pos, true, n);

                                input.drain(pos - 2 ..= pos + 1);
                                input.insert(pos - 2, Element::Num(0));

                                on_explode = false;
                                consecutive_fails = 0;
                                break;
                            }
                        }
                    } else {
                        if num >= 10 {
                            // split
                            let mut to_insert: Vec<Element> = Vec::new();
                            let float = (num as f32) / 2.0;
                            to_insert.push(Element::Open);
                            to_insert.push(Element::Num(float.floor() as u8));
                            to_insert.push(Element::Num(float.ceil() as u8));
                            to_insert.push(Element::Close);

                            input.splice(pos - 1 ..= pos - 1, to_insert);

                            on_explode = false;
                            consecutive_fails = 0;
                            break;
                        }
                    }
                }
            }
        }

        if pos == input.len() {
            consecutive_fails += 1;

            if consecutive_fails > 10 { return; }
        }

    }
}

fn read_magnitude(input: &Vec<Element>, pos: &mut usize) -> u32 {
    let mut magnitude = 0;
    let mut left = true;

    loop {
        let el = input[*pos];
        *pos += 1;

        match el {
            Element::Open => {
                let elm = read_magnitude(input, pos);
                magnitude += elm * if left { 3 } else { 2 };
                left = false;
            },
            Element::Num(n) => {
                magnitude += n as u32 * if left { 3 } else { 2 };
                left = false;
            }
            Element::Close => break
        }
    }

    magnitude
}

#[allow(dead_code)]
fn pretty_print(input: &Vec<Element>) {
    for i in 0 .. input.len() - 1 {
        let el = input[i];
        let n = input[i + 1];

        match el {
            Element::Open => print!("["),
            Element::Close => {
                print!("]");
                if n != Element::Close {
                    print!(",");
                }
            },
            Element::Num(num) => {
                print!("{}", num);
                if n != Element::Close {
                    print!(",");
                }
            }
        }
    }

    print!("]\n");
}

fn part1(input: &String) -> EmptyResult {
    let lines: Vec<&str> = input.lines().collect();
    let mut num = parse(lines[0]);

    // add snailfish numbers
    for i in 1 .. lines.len() {
        // append next snailfish number
        num.insert(0, Element::Open);
        num.append(&mut parse(lines[i]));
        num.push(Element::Close);

        // reduce
        reduce(&mut num);
    }

    // calculate magnitude
    println!("part 1: {}", read_magnitude(&num, &mut 1));
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let lines: Vec<&str> = input.lines().collect();
    let mut largest = 0;

    for i in 0 .. lines.len() {
        for j in 0 .. lines.len() {
            if i == j { continue; }

            let mut a = parse(lines[i]);
            a.insert(0, Element::Open);
            a.append(&mut parse(lines[j]));
            a.push(Element::Close);

            reduce(&mut a);
            largest = std::cmp::max(read_magnitude(&a, &mut 1), largest);
        }
    }

    println!("part 2: {}", largest);
    Ok(())
}
