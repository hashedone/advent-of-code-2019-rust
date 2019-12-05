use async_std;
use async_std::prelude::*;
use async_std::stream::Stream;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Read,
    Write,
    JmpT,
    JmpF,
    Less,
    Equal,
}

impl Op {
    fn new(code: i64) -> Self {
        match code % 100 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Read,
            4 => Self::Write,
            5 => Self::JmpT,
            6 => Self::JmpF,
            7 => Self::Less,
            8 => Self::Equal,
            c => panic!("Invalid opcode: {}", c),
        }
    }

    async fn perform<S: Stream<Item = String> + Unpin>(
        &self,
        args: &[Argument],
        memory: &mut [i64],
        input: &mut S,
    ) -> Option<usize> {
        match self {
            Self::Add => {
                args[2].set(memory, args[0].get(memory) + args[1].get(memory));
                None
            }
            Self::Mul => {
                args[2].set(memory, args[0].get(memory) * args[1].get(memory));
                None
            }
            Self::Read => {
                args[0].set(memory, input.next().await.unwrap().parse().unwrap());
                None
            }
            Self::Write => {
                async_std::println!("{}", args[0].get(memory)).await;
                None
            }
            Self::JmpT => {
                if args[0].get(memory) != 0 {
                    Some(args[1].get(memory) as usize)
                } else {
                    None
                }
            }
            Self::JmpF => {
                if args[0].get(memory) == 0 {
                    Some(args[1].get(memory) as usize)
                } else {
                    None
                }
            }
            Self::Less => {
                if args[0].get(memory) < args[1].get(memory) {
                    args[2].set(memory, 1);
                } else {
                    args[2].set(memory, 0);
                }
                None
            }
            Self::Equal => {
                if args[0].get(memory) == args[1].get(memory) {
                    args[2].set(memory, 1);
                } else {
                    args[2].set(memory, 0);
                }
                None
            }
        }
    }

    fn args(&self) -> usize {
        match self {
            Self::Add | Self::Mul | Self::Less | Self::Equal => 3,
            Self::Read | Self::Write => 1,
            Self::JmpT | Self::JmpF => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Imm(i64),
    Pos(usize),
}

impl Argument {
    fn get(&self, memory: &[i64]) -> i64 {
        match self {
            Self::Imm(v) => *v,
            Self::Pos(a) => memory[*a],
        }
    }

    fn set(&self, memory: &mut [i64], val: i64) {
        match self {
            Self::Imm(_) => panic!("Trying to output to immediate argument"),
            Self::Pos(a) => memory[*a] = val,
        }
    }
}

// Returns offset to next instruction or None
// for stop execution
async fn handle_opcode<S: Stream<Item = String> + Unpin>(
    pc: usize,
    memory: &mut [i64],
    input: &mut S,
) -> Option<usize> {
    let mut opcode = memory[pc];

    if opcode == 99 {
        return None;
    }

    let op = Op::new(opcode);
    opcode /= 100;

    let mut args = [Argument::Imm(0); 3];
    for (i, arg) in (0..op.args()).zip(args.iter_mut()) {
        let v = memory[pc + i + 1];

        *arg = if opcode % 10 == 0 {
            Argument::Pos(v as usize)
        } else {
            Argument::Imm(v)
        };

        opcode /= 10;
    }

    Some(
        op.perform(&args, memory, input)
            .await
            .unwrap_or_else(|| pc + op.args() + 1),
    )
}

async fn interpret<S: Stream<Item = String> + Unpin>(mut program: Vec<i64>, input: &mut S) -> i64 {
    let mut pc = Some(0);
    while let Some(p) = pc {
        pc = handle_opcode(p, &mut program, input).await
    }
    program[0]
}

async fn parse_program<S: Stream<Item = String> + Unpin>(input: &mut S) -> Vec<i64> {
    input
        .next()
        .await
        .unwrap()
        .split(',')
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> () {
    let mut program = parse_program(&mut input).await;
    interpret(program, &mut input).await;
}

