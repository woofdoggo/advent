const TX1: i32 = 32;
const TX2: i32 = 65;
const TY1: i32 = -225;
const TY2: i32 = -177;

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    part1()?;
    part2()?;

    Ok(())
}

fn solve(xv_orig: i32, yv_orig: i32) -> i32 {
    let mut highest_y = i32::MIN;

    let mut x = 0;
    let mut y = 0;
    let mut xv = xv_orig;
    let mut yv = yv_orig;
    for _ in 0 .. 1000 {
        x += xv;
        y += yv;

        highest_y = std::cmp::max(highest_y, y);
        if x >= TX1 && x <= TX2 && y >= TY1 && y <= TY2 { 
            return highest_y;
        }

        xv -= xv.signum();
        yv -= 1;

    }

    i32::MIN
}

fn part1() -> EmptyResult {
    let mut highest_y = 1;

    for xv in -500 .. 500 {
        for yv in 0 .. 500 {
            highest_y = std::cmp::max(highest_y, solve(xv, yv));
        }
    }

    println!("part 1: {}", highest_y);
    Ok(())
}

fn part2() -> EmptyResult {
    let mut solutions = 0;

    for xv in -500 .. 500 {
        for yv in -500 .. 500 {
            if solve(xv, yv) > i32::MIN {
                solutions += 1;
            }
        }
    }

    println!("part 2: {}", solutions);
    Ok(())
}
