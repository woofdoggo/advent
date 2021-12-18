use std::io::{self, Read};

type Octopi = [[u8; 10]; 10];
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

const POS_MOD: [[i8; 2]; 8] = [
    [1, -1],
    [1, 0],
    [1, 1],
    [0, -1],
    [0, 1],
    [-1, -1],
    [-1, 0],
    [-1, 1]
];

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> Octopi {
    let mut res: Octopi = [[0; 10]; 10];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            res[i][j] = char.to_digit(10).unwrap() as u8;
        }
    }

    res
}

fn part1(input: &String) -> EmptyResult {
    let mut grid = parse(input);
    let mut flashes: Vec<(usize, usize)> = Vec::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut flash_count: u32 = 0;

    // 100 steps
    for _ in 0 .. 100 {
        // clear flashes vec
        flashes.clear();

        // increase all energy levels by 1
        for i in 0 .. 10 {
            for j in 0 .. 10 {
                grid[i][j] += 1;
                if grid[i][j] > 9 && !flashes.contains(&(i, j)) {
                    queue.push((i, j));
                }

                while queue.len() > 0 {
                    let el = queue.pop().unwrap();

                    if flashes.contains(&el) { continue; }

                    flashes.push(el);
                    for pos in POS_MOD {
                        let ix = el.0 as i8;
                        let iy = el.1 as i8;
                        let tp = (ix + pos[0], iy + pos[1]);

                        if tp.0 < 0 || tp.1 < 0 || tp.0 == 10 || tp.1 == 10 {
                            continue;
                        }

                        let tp = (tp.0 as usize, tp.1 as usize);
                        grid[tp.0][tp.1] += 1;

                        if grid[tp.0][tp.1] > 9 {
                            queue.push((tp.0, tp.1));
                        }
                    }
                }
            }
        }

        for flash in &flashes {
            grid[flash.0][flash.1] = 0;
            flash_count += 1;
        }
    }

    println!("part 1: {}", flash_count);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let mut grid = parse(input);
    let mut flashes: Vec<(usize, usize)> = Vec::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();

    for idx in 0 .. 100000 {
        // clear flashes vec
        flashes.clear();

        // increase all energy levels by 1
        for i in 0 .. 10 {
            for j in 0 .. 10 {
                grid[i][j] += 1;
                if grid[i][j] > 9 && !flashes.contains(&(i, j)) {
                    queue.push((i, j));
                }

                while queue.len() > 0 {
                    let el = queue.pop().unwrap();

                    if flashes.contains(&el) { continue; }

                    flashes.push(el);
                    for pos in POS_MOD {
                        let ix = el.0 as i8;
                        let iy = el.1 as i8;
                        let tp = (ix + pos[0], iy + pos[1]);

                        if tp.0 < 0 || tp.1 < 0 || tp.0 == 10 || tp.1 == 10 {
                            continue;
                        }

                        let tp = (tp.0 as usize, tp.1 as usize);
                        grid[tp.0][tp.1] += 1;

                        if grid[tp.0][tp.1] > 9 {
                            queue.push((tp.0, tp.1));
                        }
                    }
                }
            }
        }

        if flashes.len() == 100 {
            println!("part 2: {}", idx + 1);
            return Ok(());
        }

        for flash in &flashes {
            grid[flash.0][flash.1] = 0;
        }
    }

    Ok(())
}
