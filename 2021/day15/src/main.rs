use std::{io::{self, Read}, collections::VecDeque};

type Position = (usize, usize, i32);
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> (Vec<Vec<u8>>, Vec<Vec<i32>>) {
    let mut result = Vec::new();
    let mut depth = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as u8);
            row2.push(-1);
        }
        result.push(row);
        depth.push(row2);
    }

    (result, depth)
}

fn clone_map(input: &Vec<Vec<u8>>, increment: u8) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for line in input {
        let mut row = Vec::new();
        for cell in line {
            let mut new_val = cell + increment;
            if new_val > 9 {
                new_val -= 9;
            }

            row.push(new_val);
        }
        result.push(row);
    }

    result
}

fn tile_map(input: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, Vec<Vec<i32>>) {
    // generate blank map
    let mut result = Vec::new();
    let mut depth = Vec::new();

    for _ in 0 .. input.len() * 5 {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        for _ in 0 .. input.len() * 5 {
            row.push(0);
            row2.push(-1);
        }
        result.push(row);
        depth.push(row2);
    }


    // fill map
    for x in 0 .. 5 {
        for y in 0 .. 5 {
            let (ox, oy) = (input.len() * x, input.len() * y);
            let curr = clone_map(&input, (x + y) as u8);

            for i in 0 .. input.len() {
                for j in 0 .. input.len() {
                    result[i + ox][j + oy] = curr[i][j];
                }
            }
        }
    }

    (result, depth)
}

fn solve(map: &Vec<Vec<u8>>, depth_map: &mut Vec<Vec<i32>>) -> () {
    let mut queue: VecDeque<Position> = VecDeque::new();
    
    queue.insert(queue.len(), (0, 0, 0));
    while queue.len() > 0 {
        let el = queue.pop_front().unwrap();
        let (x, y) = (el.0, el.1);
        depth_map[el.0][el.1] = el.2;

        add_queue(&map, &depth_map, &mut queue, (x + 1, y, el.2));
        add_queue(&map, &depth_map, &mut queue, (x, y + 1, el.2));
    }
}

fn add_queue(map: &Vec<Vec<u8>>, depth: &Vec<Vec<i32>>, queue: &mut VecDeque<Position>, pos: Position) {
    if pos.0 > depth.len() - 1 || pos.1 > depth[0].len() - 1 {
        return;
    }

    let pos_depth = pos.2 + map[pos.0][pos.1] as i32;

    match queue.iter().position(|el| el.0 == pos.0 && el.1 == pos.1) {
        Some(idx) => {
            if queue[idx].2 > pos_depth {
                queue[idx].2 = pos_depth;
            }
        },
        None => {
            let d = depth[pos.0 as usize][pos.1 as usize];
            if d == -1 || d > pos_depth {
                queue.insert(queue.len(), (pos.0, pos.1, pos_depth));
            }
        }
    }
}

fn part1(input: &String) -> EmptyResult {
    let (map, mut depth) = parse(input);
    solve(&map, &mut depth);

    println!("part 1: {}", depth.last().unwrap().last().unwrap());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let (orig_map, _) = parse(input);
    let (map, mut depth) = tile_map(&orig_map);
    solve(&map, &mut depth);

    println!("part 2: {}", depth.last().unwrap().last().unwrap());
    Ok(())
}
