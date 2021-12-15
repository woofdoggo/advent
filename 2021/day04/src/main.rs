use std::io::{self, Read};

type Board = [[u8; 5]; 5];
type EmptyResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_board(input: &[&str]) -> Board {
    let mut board = [[0u8; 5]; 5];

    for (i, line) in input.iter().enumerate() {
        let split: Vec<&str> = line.split_whitespace().collect();
        for (j, strnum) in split.iter().enumerate() {
            board[i][j] = strnum.parse::<u8>().unwrap();
        }
    }

    board
}

fn check_board(input: &Board, marks: &[u8]) -> bool {
    for i in 0 .. 5 {
        let mut row = true;
        let mut col = true;

        for j in 0 .. 5 {
            if !marks.contains(&input[i][j]) {
                row = false;
            }

            if !marks.contains(&input[j][i]) {
                col = false;
            }
        }

        if row || col {
            return true;
        }
    }

    false
}

fn calculate_sum(input: &Board, marks: &[u8], last: u8) -> u32 {
    let mut sum: u32 = 0;

    for row in input.iter() {
        for cell in row.iter() {
            if !marks.contains(cell) {
                sum += *cell as u32;
            }
        }
    }

    sum * last as u32
}

fn parse(input: &String) -> (Vec<Board>, Vec<u8>) {
    // parse draw order
    let lines: Vec<&str> = input.lines().collect();
    let draw_order: Vec<u8> = lines.first()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    // parse bingo boards
    let mut boards: Vec<Board> = Vec::new();
    let mut lnum: usize = 2;

    while lnum < lines.len() {
        if lines[lnum].is_empty() { 
            lnum += 1;
            continue; 
        }

        boards.push(parse_board(&lines[lnum .. lnum + 5]));
        lnum += 5;
    }

    (boards, draw_order)
}

fn part1(input: &String) -> EmptyResult {
    let (boards, draw_order) = parse(input);

    // simulate until winner is found
    for i in 0 .. draw_order.len() {
        let marks = &draw_order[0 .. i];
        for board in &boards {
            if check_board(&board, &marks) {
                // winner. calculate sum
                println!("winner: {}", calculate_sum(
                        &board, &marks, *marks.last().unwrap()));

                return Ok(());
            }
        }
    }

    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let (boards, draw_order) = parse(input);
    let mut drawn_boards: Vec<&Board> = Vec::new();

    for i in 0 .. draw_order.len() {
        let marks = &draw_order[0 .. i];
        for board in &boards {
            if check_board(&board, &marks) && !drawn_boards.contains(&board) {
                drawn_boards.push(&board);
                if drawn_boards.len() == boards.len() {
                    println!("loser: {}", calculate_sum(
                        &board,
                        &marks,
                        *marks.last().unwrap()
                    ));
                    return Ok(());
                }
            }
        }
    }



    Ok(())
}
