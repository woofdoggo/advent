use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Enhancement = [bool; 512];

#[derive(Debug)]
struct Image {
    points: Vec<Vec<bool>>,
    bg: bool
}

impl Image {
    fn new(size: usize, bg: bool) -> Image {
        let points = vec![vec![bg; size]; size];
        Image { points, bg }
    }

    fn get_pixel(&self, x: isize, y: isize) -> bool {
        let len = self.points.len() as isize - 1;

        if x > 0 && y > 0 && x < len && y < len {
            self.points[x as usize][y as usize]
        } else {
            self.bg
        }
    }
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse(input: &String) -> (Enhancement, Image) {
    let lines: Vec<&str> = input.lines().collect();

    // read enhancement algorithm
    let enhance = lines.first().unwrap();
    let mut enhancement: Enhancement = [false; 512];

    for (i, c) in enhance.char_indices() {
        match c {
            '#' => enhancement[i] = true,
            _ => ()
        }
    }

    // read image points
    const PADDING_SIZE: usize = 200;
    let mut img = Image::new(lines[2].len() + PADDING_SIZE * 2, false);
    let image = &mut img.points;

    for y in 2 .. lines.len() {
        let line = lines[y];

        // read row
        for (i, c) in line.char_indices() {
            match c {
                '#' => image[i + PADDING_SIZE][y + PADDING_SIZE] = true,
                _ => image[i + PADDING_SIZE][y + PADDING_SIZE] = false,
            };
        }
    }

    (enhancement, img)
}

fn iter(algo: &Enhancement, img: Image) -> Image {
    // this doesn't work with sample (forced bg flip)
    // just remove the negation operator
    // i hate these one-off things in the input
    let mut out = Image::new(img.points.len(), img.bg);

    for i in 0 .. img.points.len() {
        for j in 0 .. img.points.len() {
            let ii = i as isize;
            let ji = j as isize;

            let mut num = 0;
            if img.get_pixel(ii + 1, ji + 1) { num |= 1; }
            if img.get_pixel(ii,     ji + 1) { num |= 1 << 1; }
            if img.get_pixel(ii - 1, ji + 1) { num |= 1 << 2; }
            if img.get_pixel(ii + 1, ji)     { num |= 1 << 3; }
            if img.get_pixel(ii,     ji)     { num |= 1 << 4; }
            if img.get_pixel(ii - 1, ji)     { num |= 1 << 5; }
            if img.get_pixel(ii + 1, ji - 1) { num |= 1 << 6; }
            if img.get_pixel(ii,     ji - 1) { num |= 1 << 7; }
            if img.get_pixel(ii - 1, ji - 1) { num |= 1 << 8; }
            out.points[i][j] = algo[num];
        }
    }

    out
}

fn sum_pixels(img: &Image) -> u32 {
    let mut count = 0;

    for i in 0 .. img.points.len() {
        for j in 0 .. img.points.len() {
            if img.points[i][j] { count += 1; }
        }
    }

    count
}

fn display(img: &Image) {
    for i in 0 .. img.points.len() {
        for j in 0 .. img.points.len() {
            if img.get_pixel(j as isize, i as isize) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn part1(input: &String) -> EmptyResult {
    let (algo, mut img) = parse(input);

    for _ in 0 .. 2 {
        img = iter(&algo, img);
    }

    println!("part 1: {}", sum_pixels(&img));
    Ok(())
}

fn part2(input: &String) -> EmptyResult {
    let (algo, mut img) = parse(input);

    for _ in 0 .. 50 {
        img = iter(&algo, img);
    }

    println!("part 2: {}", sum_pixels(&img));
    Ok(())
}
