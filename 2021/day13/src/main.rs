use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> EmptyResult {
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

    
    for fold in folds {
    let mut dots_new: Vec<(usize, usize)> = Vec::new();
    if fold.0 {
        // x fold (left)
        for dot in dots {
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
        for dot in dots {
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

    println!("");
    for i in 0 .. 12 {
        for j in 0 .. 12 {
            let mut found = false;
            for point in &dots {
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

    println!("part 1: {}", dots.len());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
