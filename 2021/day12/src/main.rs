use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

struct Pathfinder {
    connections: Vec<(usize, usize)>,
    names: Vec<String>,
    paths: Vec<Vec<usize>>,

    start: usize,
    end: usize,

    cache: Vec<Vec<usize>>
}

impl Pathfinder {
    fn new(connections: Vec<(usize, usize)>, names: Vec<String>, paths: Vec<Vec<usize>>) -> Self {
        let start = names.iter().position(|el| el == &"start").unwrap();
        let end = names.iter().position(|el| el == &"end").unwrap();
        let cache: Vec<Vec<usize>> = Vec::with_capacity(names.len());

        let mut pf: Pathfinder = Pathfinder { 
            connections, names, paths, start, end,
            cache
        };

        pf.init_cache();
        pf
    }

    fn init_cache(&mut self) {
        for _ in 0 .. self.cache.capacity() { self.cache.push(Vec::new()); }

        for conn in &self.connections {
            self.cache[conn.0].push(conn.1);
            self.cache[conn.1].push(conn.0);
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

fn parse(input: &String) -> (Vec<(usize, usize)>, Vec<String>) {
    let mut connections: Vec<(usize, usize)> = Vec::new();
    let mut names: Vec<&str> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        if !names.contains(&parts[0]) { names.push(&parts[0]); }
        if !names.contains(&parts[1]) { names.push(&parts[1]); }
        
        connections.push((
            names.iter().position(|el| el == &parts[0]).unwrap(),
            names.iter().position(|el| el == &parts[1]).unwrap()
        ));
    }

    (connections, names.iter().map(|el| {
        el.to_string()
    }).collect::<Vec<String>>())
}

fn recurse(pf: &mut Pathfinder, path: Vec<usize>) {
    if *path.last().unwrap() == pf.end {
        pf.paths.push(path.clone());
        return;
    }

    // circumventing the borrow checker. woo
    let conns = &pf.cache[*path.last().unwrap()].clone();

    for conn in conns {
        // path forward
        let name = &pf.names[*conn];
        if name.chars().next().unwrap().is_lowercase() {
            if path.contains(&conn) {
                continue;
            }
        }

        let mut new_path = path.clone();
        new_path.push(*conn);
        recurse(pf, new_path);
    }
}

fn part1(input: &String) -> EmptyResult {
    let (connections, names) = parse(input);
    let paths: Vec<Vec<usize>> = Vec::new();

    let mut pf = Pathfinder::new(connections, names, paths);
    let mut path: Vec<usize> = Vec::new();
    path.push(pf.start);
    recurse(&mut pf, path);

    println!("part 1: {}", pf.paths.len());
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
