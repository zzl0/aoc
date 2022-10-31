use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    operand: isize,
}

type Program = Vec<Instruction>;

#[derive(Debug, Clone, Copy, Default)]
struct State {
    pc: usize,
    acc: isize,
}

impl State {
    fn next(self, program: &Program) -> Option<Self> {
        if !(0..program.len()).contains(&self.pc) {
            return None;
        }

        let ins = program[self.pc];
        Some(match ins.kind {
            InstructionKind::Nop => Self {
                pc: self.pc + 1,
                ..self
            },
            InstructionKind::Acc => Self {
                pc: self.pc + 1,
                acc: self.acc + ins.operand,
            },
            InstructionKind::Jmp => Self {
                pc: (self.pc as isize + ins.operand).try_into().unwrap(),
                ..self
            },
        })
    }
}

fn parse_program(input: &str) -> Program {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');
            let kind = match tokens.next() {
                Some(tok) => match tok {
                    "nop" => InstructionKind::Nop,
                    "acc" => InstructionKind::Acc,
                    "jmp" => InstructionKind::Jmp,
                    _ => panic!("unknown instruction kind {}", tok),
                },
                None => panic!("for line {}, expected instruction kind", line),
            };
            let operand = match tokens.next() {
                Some(tok) => tok.parse().unwrap(),
                None => panic!("for line {}, expected operand", line),
            };
            Instruction { kind, operand }
        })
        .collect()
}

fn part1(program: &Program) {
    let mut iter = itertools::iterate(State::default(), |s| s.next(program).unwrap());
    let mut set: HashSet<usize> = Default::default();
    let answer = iter.find(|state| !set.insert(state.pc)).unwrap();
    println!("part1: {:?}", answer);
}

fn find_variant(program: &Program) {
    let mut variants: Vec<_> = program
        .iter()
        .enumerate()
        .filter_map(|(index, ins)| match ins.kind {
            InstructionKind::Jmp | InstructionKind::Nop => Some(index),
            _ => None,
        })
        .map(|i| {
            let mut variant = program.clone();
            flip_kind(&mut variant[i].kind);
            (i, variant)
        })
        .map(|(index, variant)| {
            itertools::iterate(Some(State::default()), move |state| {
                state
                    .unwrap_or_else(|| panic!("variant {} terminated", index))
                    .next(&variant)
            })
        })
        .collect();

    loop {
        for v in &mut variants {
            v.next();
        }
    }
}

fn flip_kind(kind: &mut InstructionKind) {
    *kind = match *kind {
        InstructionKind::Jmp => InstructionKind::Nop,
        InstructionKind::Nop => InstructionKind::Jmp,
        x => x,
    }
}

fn eval(program: &Program) -> Option<isize> {
    itertools::iterate(Some(State::default()), |state| {
        state.and_then(|state| state.next(program))
    })
    .while_some()
    .last()
    .map(|s| s.acc)
}

fn part2(program: &mut Program) {
    // find_variant(program);
    flip_kind(&mut program[195].kind);
    dbg!(eval(program));
}

fn main() {
    let input = include_str!("../../../data/day08.txt");
    let mut program = parse_program(input);

    part1(&program);
    part2(&mut program);
}
