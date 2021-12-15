use std::io::{self, Read};

type Seafloor = [[u8; 1000]; 1000];
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> EmptyResult {
    // parse seafloor map
    let mut map: Seafloor = [[0; 1000]; 1000];
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut start = words.first().expect("fail").split(',');
        let mut end = words.last().expect("fail").split(',');

        let x1 = start.next().unwrap().parse::<usize>().unwrap();
        let y1 = start.last().unwrap().parse::<usize>().unwrap();
        let x2 = end.next().unwrap().parse::<usize>().unwrap();
        let y2 = end.last().unwrap().parse::<usize>().unwrap();

        let (startx, starty, endx, endy): (usize, usize, usize, usize);

        if x1 < x2 { 
            startx = x1;
            endx = x2;
        } else {
            startx = x2;
            endx = x1;
        }

        if y1 < y2 {
            starty = y1;
            endy = y2;
        } else {
            starty = y2;
            endy = y1;
        }

        for i in 10 .. 1 {
            println!("{}", i);
        }

        if startx == endx {
            for i in starty ..= endy {
                map[startx][i] += 1;
            }
        } else if starty == endy {
            for i in startx ..= endx {
                map[i][starty] += 1;
            }
        }
    }

    // scan
    let mut spots: u32 = 0;
    for i in 0 .. 1000 {
        for j in 0 .. 1000 {
            if map[i][j] > 1 {
                spots += 1;
            }
        }
    }

    println!("part 1: {}", spots);
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
