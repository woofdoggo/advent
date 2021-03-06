use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn fold(dots: Vec<(usize, usize)>, folds: &Vec<(bool, usize)>) -> Vec<(usize, usize)> {
    let mut dots_new: Vec<(usize, usize)>;
    let mut dots = dots.clone();

    for fold in folds {
        dots_new = Vec::new();
        if fold.0 {
            // x fold (left)
            for dot in &dots {
                if dot.0 < fold.1 { 
                    let newdot = (dot.0, dot.1);
                    if !dots_new.contains(&newdot) { dots_new.push(newdot); }
                } else {
                    let newdot = (fold.1 - (dot.0 - fold.1), dot.1);
                    if !dots_new.contains(&newdot) { dots_new.push(newdot); }
                }
            }
        } else {
            // y fold (up)
            for dot in &dots {
                if dot.1 < fold.1 { 
                    let newdot = (dot.0, dot.1);
                    if !dots_new.contains(&newdot) { dots_new.push(newdot); }
                } else {
                    let newdot = (dot.0, fold.1 - (dot.1 - fold.1));
                    if !dots_new.contains(&newdot) { dots_new.push(newdot); }
                }
            }
        }

        dots = dots_new;
    }

    dots
}

fn display(dots: &Vec<(usize, usize)>) {
    for i in 0 .. 40 {
        for j in 0 .. 80 {
            let mut found = false;
            for point in dots {
                if point.0 == j && point.1 == i {
                    found = true;
                    break;
                }
            }

            if found { print!("#"); } else { print!("."); }
        }
        print!("\n");
    }
}

fn parse(input: &String) -> (Vec<(usize, usize)>, Vec<(bool, usize)>) {
    let mut dots: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(bool, usize)> = Vec::new();

    let mut parse_a = true;
    for line in input.lines() {
        if line.is_empty() {
            parse_a = false;
            continue;
        }

        if parse_a {
            // parse dot
            let mut parts = line.split(',');
            let a = parts.next().unwrap().parse::<usize>().unwrap();
            let b = parts.next().unwrap().parse::<usize>().unwrap();
            dots.push((a, b));
        } else {
            // parse fold
            let parts = line.split_whitespace();
            let mut parts = parts.last().unwrap().split('=');
            let dir: bool = parts.next().unwrap().chars().next().unwrap() == 'x';
            let num = parts.next().unwrap().parse::<usize>().unwrap();
            folds.push((dir, num));
        }
    }

    (dots, folds)
}

fn part1(input: &String) -> EmptyResult {
    let (dots, folds) = parse(input);
    let dots = fold(dots, &folds[..1].to_vec());

    println!("part 1: {}", dots.len());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let (dots, folds) = parse(input);
    let dots = fold(dots, &folds);

    println!("part 2:");
    display(&dots);
    Ok(())
}
