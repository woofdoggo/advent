use std::collections::HashMap;

const SAMPLE_INPUT: [u32; 2] = [4, 8];
const ACTUAL_INPUT: [u32; 2] = [1, 5];

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Current {
    A, B
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Player {
    position: u32,
    score: u32
}

impl Player {
    fn move_forwards(&mut self, spaces: u32) {
        self.position += spaces;
        while self.position > 10 {
            self.position -= 10;
        }

        self.score += self.position;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GameState {
    dice_rolls: u32,
    current_player: Current,
    a: Player,
    b: Player
}

impl GameState {
    fn roll_thrice(&mut self) -> u32 {
        let mut sum = 0;
        for _ in 0 .. 3 {
            self.dice_rolls += 1;
            sum += if self.dice_rolls % 100 != 0 {
                self.dice_rolls % 100
            } else {
                100
            };
        }

        sum
    }

    /// Returns true if the game ends, otherwise false.
    fn turn(&mut self) -> bool {
        let movement = self.roll_thrice();

        match self.current_player {
            Current::A => {
                self.current_player = Current::B;
                self.a.move_forwards(movement);
            },
            Current::B => {
                self.current_player = Current::A;
                self.b.move_forwards(movement);
            }
        };

        self.a.score >= 1000 || self.b.score >= 1000
    }
}

fn main() -> EmptyResult {
    // this input is so simple that i cant be bothered
    // to parse it
    let input = ACTUAL_INPUT;

    let state = GameState {
        dice_rolls: 0,
        current_player: Current::A,
        a: Player {
            position: input[0],
            score: 0
        },
        b: Player {
            position: input[1],
            score: 0
        }
    };

    part1(state.clone())?;
    part2(state.clone())?;

    Ok(())
}

fn part1(mut input: GameState) -> EmptyResult {
    loop {
        if input.turn() { break; }
    }

    let output = std::cmp::min(input.a.score, input.b.score) * input.dice_rolls;
    println!("part 1: {}", output);
    Ok(())
}

fn part2(input: GameState) -> EmptyResult {
    let mut universes: HashMap<GameState, u128> = HashMap::new();
    universes.insert(input, 1);

    let mut wins_a: u128 = 0;
    let mut wins_b: u128 = 0;

    loop {
        let mut new_universes: HashMap<GameState, u128> = HashMap::new();

        for (k, v) in universes.iter() {
            for a in 1 ..= 3 {
                for b in 1 ..= 3 {
                    for c in 1 ..= 3 {
                        let mut new_state = k.clone();
                        let player = match new_state.current_player {
                            Current::A => {
                                new_state.current_player = Current::B;
                                &mut new_state.a
                            },
                            Current::B => {
                                new_state.current_player = Current::A;
                                &mut new_state.b
                            }
                        };

                        player.move_forwards(a + b + c);
                        
                        if new_state.a.score >= 21 {
                            wins_a += v;
                            continue;
                        } else if new_state.b.score >= 21 {
                            wins_b += v;
                            continue;
                        }

                        *new_universes.entry(new_state).or_insert(0) += v;
                    }
                }
            }
        }

        if new_universes.len() == 0 { break; }
        universes = new_universes;
    }

    println!("part 2: {}", std::cmp::max(wins_a, wins_b));
    Ok(())
}
