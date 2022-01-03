mod test;

use std::io::{self, Read};

type EmptyResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
enum Register {
    X, W, Y, Z
}

#[derive(Clone, Copy, Debug)]
enum Source {
    Reg(Register),
    Const(i32)
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
    input: Vec<i32>,
    input_ptr: usize,
    w: i32,
    x: i32,
    y: i32,
    z: i32
}

impl<'a> ALU<'a> {
    fn run_program(instructions: &Vec<Instruction>, input: Vec<i32>) -> [i32; 4] {
        let mut alu = ALU {
            instructions,
            inst_ptr: 0,
            input,
            input_ptr: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0
        };

        while alu.inst_ptr < alu.instructions.len() {
            alu.cycle();
        }

        [alu.w, alu.x, alu.y, alu.z]
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

    fn read(&self, src: Source) -> i32 {
        match src {
            Source::Reg(reg) => self.read_register(reg),
            Source::Const(int) => int
        }
    }

    fn read_register(&self, register: Register) -> i32 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z
        }
    }

    fn set_register(&mut self, register: Register, value: i32) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value
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

fn str(input: &str) -> Register {
    match input.chars().nth(0).unwrap() {
        'w' => Register::W,
        'x' => Register::X,
        'y' => Register::Y,
        'z' => Register::Z,
        _ => panic!("invalid register")
    }
}

fn sti(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
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

fn parse(input: &String) -> Vec<Instruction> {
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

fn part1(input: &String) -> EmptyResult {
    let instrs = parse(input);

    Ok(())
}

fn part2(input: &String) -> EmptyResult {

    Ok(())
}
