use im::HashMap;
use std::fmt;

struct Program {
    instructions: Vec<Instruction>,
}

enum Instruction {
    SetMask(Mask),
    Assign { addr: u64, val: u64 },
}

#[derive(Clone, Copy, Default)]
struct Mask {
    set: u64,
    clear: u64,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::SetMask(mask) => {
                write!(f, "mask: {:?}", mask)
            }
            Instruction::Assign { addr, val } => {
                write!(f, "mem[{}] = {}", addr, val)
            }
        }
    }
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set {:036b}, clear {:036b}", self.set, self.clear)
    }
}

/*
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
*/
impl Program {
    fn parse(input: &str) -> Self {
        peg::parser! {
            pub(crate) grammar parser() for str {
                pub(crate) rule root(p: &mut Program)
                    = (line(p) whitespace()*)* ![_]

                rule line(p: &mut Program)
                    = i:instruction() { p.instructions.push(i) }

                rule instruction() -> Instruction
                    = set_mask()
                    / assign()

                rule set_mask() -> Instruction
                    = "mask = " e:$(['X' | '0' | '1']+) {
                        let mut mask: Mask = Default::default();
                        for (i, x) in e.as_bytes().iter().rev().enumerate() {
                            match x {
                                b'1' => mask.set |= 2_u64.pow(i as _),
                                b'0' => mask.clear |= 2_u64.pow(i as _),
                                _ => {},
                            }
                        }
                        Instruction::SetMask(mask)
                    }

                rule assign() -> Instruction
                    = "mem[" addr:number() "] = " val:number() { Instruction::Assign { addr, val } }

                rule number() -> u64
                    = e:$(['0'..='9']+) { e.parse().unwrap() }

                rule whitespace()
                    = [' ' | '\t' | '\r' | '\n']
            }
        }

        let mut program = Program {
            instructions: Default::default(),
        };

        parser::root(input, &mut program).unwrap();
        program
    }
}

impl Mask {
    fn apply(&self, x: u64) -> u64 {
        (x | self.set) & (!self.clear)
    }
}

fn part1(input: &str) {
    let mut mask: Mask = Default::default();
    let mut mem = HashMap::<u64, u64>::new();

    let program = Program::parse(input);
    for ins in &program.instructions {
        match *ins {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::Assign { addr, val } => {
                mem.insert(addr, mask.apply(val));
            }
        }
    }
    println!("part1: {}", mem.values().sum::<u64>());
}

fn main() {
    let input = include_str!("../../../data/day14.txt");
    part1(input);
}
