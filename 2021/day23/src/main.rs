use std::{io::{self, Read}, hash::{Hasher, Hash}};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;
type Hallway = [Amphipod; 15];
type Room = [Amphipod; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Move {
    /// Position in hallway, move left/right
    Hallway(usize, bool),

    /// Enter given room
    Enter(usize),

    /// Leave given room
    Leave(usize)
}

impl State {
    fn enter_room(&mut self, room: usize) {
        let (room, pos) = match room {
            0 => (&mut self.a, 3),
            1 => (&mut self.b, 5),
            2 => (&mut self.c, 7),
            3 => (&mut self.d, 9),
            _ => panic!("invalid room leave")
        };

        self.energy_cost += State::get_cost(self.hallway[pos]);
        if room[1] == Amphipod::Empty {
            room[1] = self.hallway[pos];
        } else if room[0] == Amphipod::Empty {
            room[0] = self.hallway[pos];
        } else {
            panic!("cant enter room");
        }
        self.hallway[pos] = Amphipod::Empty;
    }

    fn leave_room(&mut self, room: usize) {
        let (room, pos) = match room {
            0 => (&mut self.a, 3),
            1 => (&mut self.b, 5),
            2 => (&mut self.c, 7),
            3 => (&mut self.d, 9),
            _ => panic!("invalid room leave")
        };

        if room[0] != Amphipod::Empty {
            self.hallway[pos] = room[0];
            self.energy_cost += State::get_cost(room[0]);
            room[0] = Amphipod::Empty;
        } else {
            self.hallway[pos] = room[1];
            self.energy_cost += State::get_cost(room[1]);
            room[1] = Amphipod::Empty;
        }
    }

    fn room_has_slot(&self, room: usize) -> (bool, bool) {
        let room = match room {
            0 => self.a,
            1 => self.b,
            2 => self.c,
            3 => self.d,
            _ => panic!("invalid room vacancy check")
        };

        (room[0] == Amphipod::Empty, room[1] == Amphipod::Empty)
    }

    fn is_sorted(&self) -> bool {
        for i in 0 .. 4 {
            if !self.is_room_sorted(i) {
                return false;
            }
        }

        true
    }

    fn is_room_sorted(&self, room: usize) -> bool {
        let (amphipod, room) = match room {
            0 => (Amphipod::Amber,  self.a),
            1 => (Amphipod::Bronze, self.b),
            2 => (Amphipod::Copper, self.c),
            3 => (Amphipod::Desert, self.d),
            _ => panic!("invalid room sort check")
        };

        room[0] == amphipod && room[1] == amphipod
    }

    fn list_moves(&self) -> Vec<Move> {
        let mut out = Vec::new();

        // find room leaves
        for room in 0 .. 4 {
            if !self.is_room_sorted(room) {
                out.push(Move::Leave(room));
            }
        }

        // find room enters
        for room in 0 .. 4 {
            // check if appropriate amphipod is above room
            if self.hallway[3 + room * 2] == State::get_amphipod(room) {
                // check if there is another one of the wrong amphipods in the room
                let (a, b) = self.room_has_slot(room);

                // room has no amphipods - we can move in
                if a && b {
                    out.push(Move::Enter(room));
                    continue;
                }

                if !a && !b { 
                    // room is full - we cannot move in
                    continue; 
                } else {
                    // room has an open slot, check if bottom slot is sorted properly
                    let (amphipod, actual_room) = match room {
                        0 => (Amphipod::Amber,  self.a),
                        1 => (Amphipod::Bronze, self.b),
                        2 => (Amphipod::Copper, self.c),
                        3 => (Amphipod::Desert, self.d),
                        _ => panic!("invalid room sort check")
                    };

                    // bottom slot sorted properly - we can move in
                    if actual_room[1] == amphipod {
                        out.push(Move::Enter(room));
                    }
                }
            }
        }

        // find hallway maneuvers
        // this is the tricky part.
        //
        // we want to make sure that we only add movements
        // which are actually beneficial - eg. don't make any movements
        // which immediately loop back to the previous state

        out
    }

    fn get_amphipod(idx: usize) -> Amphipod {
        match idx {
            0 => Amphipod::Amber,
            1 => Amphipod::Bronze,
            2 => Amphipod::Copper,
            3 => Amphipod::Desert,
            _ => panic!("invalid amphipod room check")
        }
    }

    fn get_cost(amphipod: Amphipod) -> u32 {
        match amphipod {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
            Amphipod::Empty => panic!("no energy cost for empty amphipod")
        }
    }
}

struct Solver {
    min_cost: u32
}

impl Solver {
    fn solve(input: State) -> u32 {
        let mut solver = Solver { min_cost: u32::MAX };

        solver.min_cost
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
        _ => panic!("invalid amphipod")
    }
}

fn parse(input: &String) -> State {
    let row_a = input.lines().nth(2).unwrap();
    let row_b = input.lines().nth(3).unwrap();

    State {
        energy_cost: 0,

        hallway: [Amphipod::Empty; 15],
        a: [cta(row_a.chars().nth(3).unwrap()), cta(row_b.chars().nth(3).unwrap())],
        b: [cta(row_a.chars().nth(5).unwrap()), cta(row_b.chars().nth(5).unwrap())],
        c: [cta(row_a.chars().nth(7).unwrap()), cta(row_b.chars().nth(7).unwrap())],
        d: [cta(row_a.chars().nth(9).unwrap()), cta(row_b.chars().nth(9).unwrap())]
    }
}

fn part1(input: &String) -> EmptyResult {
    println!("{:?}", parse(input));

    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
