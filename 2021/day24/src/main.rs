use std::fs::read_to_string;

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
enum Register {
    X, W, Y, Z
}

#[derive(Clone, Copy, Debug)]
enum Source {
    Reg(Register),
    Const(i64)
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Inp(Register),
    Add(Register, Source),
    Mul(Register, Source),
    Div(Register, Source),
    Mod(Register, Source),
    Eql(Register, Source)
}

struct ALU<'a> {
    instructions: &'a Vec<Instruction>,
    inst_ptr: usize,
    input: Vec<i64>,
    input_ptr: usize,
    w: i64,
    x: i64,
    y: i64,
    z: i64
}

impl<'a> ALU<'a> {
    fn new(instructions: &Vec<Instruction>, input: Vec<i64>) -> ALU {
        ALU {
            instructions,
            inst_ptr: 0,
            input,
            input_ptr: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0
        }
    }

    fn run(&mut self) {
        while self.inst_ptr != self.instructions.len() {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        let instr = &self.instructions[self.inst_ptr];
        self.inst_ptr += 1;

        match *instr {
            Instruction::Inp(reg) => {
                let input = self.input[self.input_ptr];
                self.input_ptr += 1;

                self.set_register(reg, input);
            },
            Instruction::Add(reg1, val) => {
                let a = self.read_register(reg1);
                let b = self.read(val);
                self.set_register(reg1, a + b);
            },
            Instruction::Mul(reg1, val) => {
                let a = self.read_register(reg1);
                let b = self.read(val);
                self.set_register(reg1, a * b);
            },
            Instruction::Div(reg1, val) => {
                let a = self.read_register(reg1);
                let b = self.read(val);
                self.set_register(reg1, a / b);
            },
            Instruction::Mod(reg1, val) => {
                let a = self.read_register(reg1);
                let b = self.read(val);
                self.set_register(reg1, a % b);
            },
            Instruction::Eql(reg1, val) => {
                let a = self.read_register(reg1);
                let b = self.read(val);
                self.set_register(reg1, match a == b {
                    true => 1,
                    false => 0
                });
            }
        }
    }

    fn read(&self, src: Source) -> i64 {
        match src {
            Source::Reg(reg) => self.read_register(reg),
            Source::Const(int) => int
        }
    }

    fn read_register(&self, register: Register) -> i64 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z
        }
    }

    fn set_register(&mut self, register: Register, value: i64) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value
        }
    }
}

fn main() -> EmptyResult {
    part1()?;
    part2()?;

    Ok(())
}

fn str(input: &str) -> Register {
    match input.chars().nth(0).unwrap() {
        'w' => Register::W,
        'x' => Register::X,
        'y' => Register::Y,
        'z' => Register::Z,
        _ => panic!("invalid register")
    }
}

fn sti(input: &str) -> i64 {
    input.parse::<i64>().unwrap()
}

fn src(input: &str) -> Source {
    match input.chars().nth(0).unwrap() {
        'w' => Source::Reg(Register::W),
        'x' => Source::Reg(Register::X),
        'y' => Source::Reg(Register::Y),
        'z' => Source::Reg(Register::Z),
        _ => Source::Const(sti(input))
    }
}

fn parse() -> Vec<Vec<Instruction>> {
    let mut out = Vec::new();

    for i in 1 ..= 14 {
        let contents = read_to_string(format!("./input/block{}.txt", i)).unwrap();
        out.push(parse_instructions(&contents));
    }

    out
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    let mut out = Vec::new();

    for line in input.lines() {
        let (instr, registers) = line.split_once(' ').unwrap();
        let registers: Vec<&str> = registers.split(' ').collect();
        out.push(match instr {
            "inp" => Instruction::Inp(str(registers[0])),
            "add" => Instruction::Add(str(registers[0]), src(registers[1])),
            "mul" => Instruction::Mul(str(registers[0]), src(registers[1])),
            "div" => Instruction::Div(str(registers[0]), src(registers[1])),
            "mod" => Instruction::Mod(str(registers[0]), src(registers[1])),
            "eql" => Instruction::Eql(str(registers[0]), src(registers[1])),
            _ => panic!("invalid instruction")
        });
    }

    out
}

/// Returns: (z, i)
fn solve(block: &Vec<Instruction>, out: i64) -> (i64, i64) {
    for i in (1 ..= 9).rev() {
        for z in 0 ..= 1000000 {
            let mut alu = ALU::new(&block, vec![i]);
            alu.z = z;
            alu.run();

            if alu.z == out {
                return (z, i);
            }
        }
    }
    
    (0,0)
}

fn part1() -> EmptyResult {
    let blocks = parse();
    let mut solves: Vec<(i64, i64)> = Vec::new();

    solves.push(solve(blocks.last().unwrap(), 0));
    for i in (0 .. blocks.len() - 1).rev() {
        solves.push(solve(&blocks[i], solves.last().unwrap().0));
    }

    println!("{:?}", solves);
    Ok(())
}

fn part2() -> EmptyResult {

    Ok(())
}
