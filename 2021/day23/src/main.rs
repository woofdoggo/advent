use std::{io::{self, Read}, hash::{Hasher, Hash}, collections::{BinaryHeap, HashMap}, cmp::Ordering};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Hallway = [Amphipod; 11];
type Room = Vec<Amphipod>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty
}

impl Amphipod {
    fn get_cost(self) -> u32 {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
            Self::Empty => panic!("tried to get cost for empty amphipod")
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct State {
    energy_cost: u32,

    hallway: Hallway,
    a: Room,
    b: Room,
    c: Room,
    d: Room
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash everything but energy_cost
        self.hallway.hash(state);
        self.a.hash(state);
        self.b.hash(state);
        self.c.hash(state);
        self.d.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway &&
        self.a == other.a &&
        self.b == other.b &&
        self.c == other.c &&
        self.d == other.d
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy_cost.cmp(&self.energy_cost)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
            .then_with(|| self.c.cmp(&other.c))
            .then_with(|| self.d.cmp(&other.d))
            .then_with(|| self.hallway.cmp(&other.hallway))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn abs_diff(a: usize, b: usize) -> u32 {
    if a > b {
        (a - b) as u32
    } else {
        (b - a) as u32
    }
}

fn hallway_cost(start: usize, dest: usize, pod_cost: u32) -> u32 {
    abs_diff(start, dest) * pod_cost
}

impl State {
    fn is_solved(&self) -> bool {
        for i in 0 .. 4 {
            if !self.is_room_sorted(i) {
                return false;
            }
        }

        true
    }

    fn generate_substates(&self) -> Vec<Self> {
        let mut out = Vec::new();

        // list of states to generate:
        // 1. move to any point on the hallway ends
        // 2. move to any point between rooms
        // 3. leave one room and enter another
        // 4. enter a room
        const VALID_HALLWAY_SLOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

        // generate actions for unsorted amphipods
        for i in 0 .. 4 {
            // skip if room is sorted or empty
            if self.is_room_sorted(i) { continue; }
            if self.get_room(i).is_empty() { continue; }
            
            // leave room -> hallway spot
            for valid_spot in VALID_HALLWAY_SLOTS {
                if self.can_go(i * 2 + 2, valid_spot) {
                    let mut new = self.clone();
                    let pod = new.leave_room(i);
                    new.hallway[valid_spot] = pod;
                    new.energy_cost += hallway_cost(i*2+2, valid_spot, pod.get_cost());

                    out.push(new);
                }
            }

            // leave room and enter another
            let pod = self.get_room(i).last().unwrap();
            let room = match *pod {
                Amphipod::Amber => 0,
                Amphipod::Bronze => 1,
                Amphipod::Copper => 2,
                Amphipod::Desert => 3,
                _ => unreachable!("cant be empty amphipod if we got one")
            };

            if self.can_go(i * 2 + 2, room * 2 + 2) {
                if self.can_enter_room(*pod) {
                    let mut new = self.clone();
                    new.leave_and_goto(i, room);

                    out.push(new);
                }
            }
        }

        // in hallway -> enter room
        for valid_spot in VALID_HALLWAY_SLOTS {
            let pod = self.hallway[valid_spot];
            let room = match pod {
                Amphipod::Amber => 0,
                Amphipod::Bronze => 1,
                Amphipod::Copper => 2,
                Amphipod::Desert => 3,
                Amphipod::Empty => continue
            };

            if self.can_go(valid_spot, room * 2 + 2) {
                if self.can_enter_room(pod) {
                    let mut new = self.clone();
                    new.hallway[valid_spot] = Amphipod::Empty;
                    new.enter_room(room, pod);
                    new.energy_cost += hallway_cost(valid_spot, room * 2 + 2, pod.get_cost());

                    out.push(new);
                }
            }
        }

        out
    }

    fn can_go(&self, start: usize, dest: usize) -> bool {
        let range = if start > dest {
            dest ..= start
        } else {
            start ..= dest 
        };

        for i in range {
            if self.hallway[i] != Amphipod::Empty {
                return false;
            }
        }

        true
    }

    fn can_enter_room(&self, pod: Amphipod) -> bool {
        let room = match pod {
            Amphipod::Amber => &self.a,
            Amphipod::Bronze => &self.b,
            Amphipod::Copper => &self.c,
            Amphipod::Desert => &self.d,
            _ => panic!("invalid can enter room check")
        };

        for el in room {
            if *el != pod {
                return false;
            }
        }

        true
    }

    fn is_room_sorted(&self, room: usize) -> bool {
        let (room, pod) = match room {
            0 => (&self.a, Amphipod::Amber),
            1 => (&self.b, Amphipod::Bronze),
            2 => (&self.c, Amphipod::Copper),
            3 => (&self.d, Amphipod::Desert),
            _ => panic!("invalid room sort check")
        };

        for el in room {
            if *el != pod {
                return false;
            }
        }

        if room.len() != 2 { return false; }
        true
    }

    fn get_room(&self, room: usize) -> &Room {
        match room {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            _ => panic!("invalid room get")
        }
    }

    fn get_room_mut(&mut self, room: usize) -> &mut Room {
        match room {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            _ => panic!("invalid room get")
        }
    }

    fn enter_room(&mut self, room: usize, pod: Amphipod) {
        let room = self.get_room_mut(room);

        if room.len() == 2 {
            panic!("room enter: room is full");
        }

        room.push(pod);

        // this will add the energy cost once if the amphipod only had
        // to move one tile down. otherwise it will add it twice
        self.energy_cost = pod.get_cost() * (3 - room.len()) as u32; 
    }

    fn leave_room(&mut self, room: usize) -> Amphipod {
        let room = self.get_room_mut(room);
        let pod = room.pop().unwrap();

        // this will add the energy const once if the amphipod only had
        // to move one tile up. otherwise it will add it twice
        self.energy_cost += pod.get_cost() * (2 - room.len()) as u32;

        pod
    }

    /// THIS ASSUMES THAT THE ROOM IS REACHABLE.
    fn leave_and_goto(&mut self, start: usize, dest: usize) {
        // leave start, enter dest
        let pod = self.leave_room(start);
        self.enter_room(dest, pod);

        // update energy cost with hallway movement
        self.energy_cost += hallway_cost(start * 2 + 2, dest * 2 + 2, pod.get_cost());
    }
}

struct Solver {
    queue: BinaryHeap<State>,
    states: HashMap<State, u32>
}

impl Solver {
    fn solve(input: State) -> u32 {
        let mut solver = Solver { 
            queue: BinaryHeap::new(),
            states: HashMap::new()
        };

        solver.queue.push(input);

        while let Some(state) = solver.queue.pop() {
            // check if state is solved
            if state.is_solved() {
                print_state(state.clone());
                return state.energy_cost;
            }
            
            // check if there is a better state
            if let Some(cost) = solver.states.get_mut(&state) {
                if *cost < state.energy_cost { 
                    continue; 
                } else {
                    *cost = state.energy_cost;
                }
            }

            let new_states = state.generate_substates();
            for state in new_states {
                if let Some(cost) = solver.states.get_mut(&state) {
                    if *cost <= state.energy_cost { 
                        continue; 
                    } else {
                        *cost = state.energy_cost;
                        solver.queue.push(state.clone());
                    }
                } else {
                    solver.queue.push(state.clone());

                    let cost = state.energy_cost;
                    solver.states.insert(state, cost);
                }
            }

            println!("queue: {} states: {} cost: {}", solver.queue.len(), solver.states.len(), state.energy_cost);
            print_state(state);
        }

        u32::MAX
    }
}

fn main() -> EmptyResult {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn cta(input: char) -> Amphipod {
    match input {
        'A' => Amphipod::Amber,
        'B' => Amphipod::Bronze,
        'C' => Amphipod::Copper,
        'D' => Amphipod::Desert,
        _ => panic!("invalid char-to-amphipod")
    }
}

fn parse(input: &String) -> State {
    let row_a = input.lines().nth(2).unwrap();
    let row_b = input.lines().nth(3).unwrap();

    State {
        energy_cost: 0,

        hallway: [Amphipod::Empty; 11],
        a: vec![cta(row_b.chars().nth(3).unwrap()), cta(row_a.chars().nth(3).unwrap())],
        b: vec![cta(row_b.chars().nth(5).unwrap()), cta(row_a.chars().nth(5).unwrap())],
        c: vec![cta(row_b.chars().nth(7).unwrap()), cta(row_a.chars().nth(7).unwrap())],
        d: vec![cta(row_b.chars().nth(9).unwrap()), cta(row_a.chars().nth(9).unwrap())]
    }
}

fn atc(i: Amphipod) -> char {
    match i {
        Amphipod::Amber => 'A',
        Amphipod::Bronze => 'B',
        Amphipod::Copper => 'C',
        Amphipod::Desert => 'D',
        Amphipod::Empty => '.'
    }
}

fn print_state(input: State) {
    println!("#############");
    print!("#");
    for i in input.hallway {
        print!("{}", atc(i));
    }
    print!("#   {}\n", input.energy_cost);

    let a = input.a;
    let b = input.b;
    let c = input.c;
    let d = input.d;
    
    print!("###");
    print!("{}", atc(if a.len() == 2 { a[1] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if b.len() == 2 { b[1] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if c.len() == 2 { c[1] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if d.len() == 2 { d[1] } else { Amphipod::Empty }));
    print!("###\n");

    print!("  #");
    print!("{}", atc(if a.len() >= 1 { a[0] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if b.len() >= 1 { b[0] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if c.len() >= 1 { c[0] } else { Amphipod::Empty }));
    print!("#");
    print!("{}", atc(if d.len() >= 1 { d[0] } else { Amphipod::Empty }));
    print!("#  \n");

    println!("  #########");
}

fn part1(input: &String) -> EmptyResult {
    println!("part 1: {}", Solver::solve(parse(input)));
    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
