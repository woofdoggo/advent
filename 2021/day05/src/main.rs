use std::io::{self, Read};

const SEAFLOOR_SIZE: usize = 1000;

type Line = (usize, usize, usize, usize);
type Seafloor = [[u8; SEAFLOOR_SIZE]; SEAFLOOR_SIZE];
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(line: &str) -> Line {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut start = words.first().expect("fail").split(',');
    let mut end = words.last().expect("fail").split(',');

    let startx = start.next().unwrap().parse::<usize>().unwrap();
    let starty = start.last().unwrap().parse::<usize>().unwrap();
    let endx = end.next().unwrap().parse::<usize>().unwrap();
    let endy = end.last().unwrap().parse::<usize>().unwrap();

    (startx, starty, endx, endy)
}

fn plot(allow_diagonals: bool, line: Line, map: &mut Seafloor) {
    let (startx, starty, endx, endy) = line;

    if startx == endx {
        let range = if starty < endy { starty ..= endy } else { endy ..= starty };
        for i in range {
            map[startx][i] += 1;
        }
    } else if starty == endy {
        let range = if startx < endx { startx ..= endx } else { endx ..= startx };
        for i in range {
            map[i][starty] += 1;
        }
    } else {
        if allow_diagonals {
            let mut x: usize = startx;
            let mut y: usize = starty;

            let xm = if startx < endx { true } else { false };
            let ym = if starty < endy { true } else { false };

            while x != endx {
                map[x][y] += 1;

                if xm { x += 1 } else { x -= 1 };
                if ym { y += 1 } else { y -= 1 };
            }

            map[x][y] += 1;
        }
    }
}

fn solve(allow_diagonals: bool, input: &String) -> u32 {
    let mut map: Seafloor = [[0; SEAFLOOR_SIZE]; SEAFLOOR_SIZE];
    for line in input.lines() {
        let (startx, starty, endx, endy) = parse(line);
        plot(allow_diagonals, (startx, starty, endx, endy), &mut map);
    }

    // scan
    let mut spots: u32 = 0;
    for i in 0 .. SEAFLOOR_SIZE {
        for j in 0 .. SEAFLOOR_SIZE {
            if map[i][j] > 1 {
                spots += 1;
            }
        }
    }

    spots
}

fn part1(input: &String) -> EmptyResult {
    println!("part 1: {}", solve(false, input));
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    println!("part 2: {}", solve(true, input));
    Ok(())
}
