use async_std;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_stream::stream;

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

    // Returns:
    // (new_pc, output_val)
    // For non jump instructions returned `new_pc` should be `None`
    // For non output instructions returned `output_val` should be `None`
    async fn perform<S: Stream<Item = i64> + Unpin>(
        &self,
        args: &[Argument],
        memory: &mut [i64],
        input: &mut S,
    ) -> (Option<usize>, Option<i64>) {
        match self {
            Self::Add => {
                args[2].set(memory, args[0].get(memory) + args[1].get(memory));
                (None, None)
            }
            Self::Mul => {
                args[2].set(memory, args[0].get(memory) * args[1].get(memory));
                (None, None)
            }
            Self::Read => {
                let readed = input.next().await.unwrap();
                args[0].set(memory, readed);
                (None, None)
            }
            Self::Write => {
                let writting = args[0].get(memory);
                (None, Some(writting))
            }
            Self::JmpT => {
                let new_pc = if args[0].get(memory) != 0 {
                    Some(args[1].get(memory) as usize)
                } else {
                    None
                };
                (new_pc, None)
            }
            Self::JmpF => {
                let new_pc = if args[0].get(memory) == 0 {
                    Some(args[1].get(memory) as usize)
                } else {
                    None
                };
                (new_pc, None)
            }
            Self::Less => {
                if args[0].get(memory) < args[1].get(memory) {
                    args[2].set(memory, 1);
                } else {
                    args[2].set(memory, 0);
                }
                (None, None)
            }
            Self::Equal => {
                if args[0].get(memory) == args[1].get(memory) {
                    args[2].set(memory, 1);
                } else {
                    args[2].set(memory, 0);
                }
                (None, None)
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

// Returns (output_val, new_pc)
// For termination opcode, returned value should be `None`
// For non output opcodes, returned `output_val` should be None
async fn handle_opcode<S: Stream<Item = i64> + Unpin>(
    pc: usize,
    memory: &mut [i64],
    input: &mut S,
) -> Option<(Option<i64>, usize)> {
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

    let (new_pc, output_val) = op.perform(&args, memory, input).await;
    let new_pc = new_pc.unwrap_or_else(|| pc + op.args() + 1);
    Some((output_val, new_pc))
}

pub fn interpret<S: Stream<Item = i64> + Unpin>(
    program: Vec<i64>,
    input: S,
) -> impl Stream<Item = i64> {
    stream!(
        let mut program = program;
        let mut input = input;
        let mut pc = 0;

        while let Some((output, new_pc)) = handle_opcode(pc, &mut program, &mut input).await {
            yield output;
            pc = new_pc;
        }
    )
    .filter_map(std::convert::identity)
}
