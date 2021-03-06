use async_std;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_stream::stream;

struct Machine {
    memory: Vec<i128>,
    relative_base: isize,
}

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
    MoveBase,
}

impl Op {
    fn new(code: i128) -> Self {
        match code % 100 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Read,
            4 => Self::Write,
            5 => Self::JmpT,
            6 => Self::JmpF,
            7 => Self::Less,
            8 => Self::Equal,
            9 => Self::MoveBase,
            c => panic!("Invalid opcode: {}", c),
        }
    }

    // Returns:
    // (new_pc, output_val)
    // For non jump instructions returned `new_pc` should be `None`
    // For non output instructions returned `output_val` should be `None`
    async fn perform<S: Stream<Item = i128> + Unpin>(
        &self,
        args: &[Argument],
        machine: &mut Machine,
        input: &mut S,
    ) -> (Option<usize>, Option<i128>) {
        match self {
            Self::Add => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "ADD   {:5?}[{:5}]  {:5?}[{:5}]  {:5?}",
                    args[0], arg1, args[1], arg2, args[2]
                );
                args[2].set(machine, arg1 + arg2);
                (None, None)
            }
            Self::Mul => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "MUL   {:5?}[{:5}]  {:5?}[{:5}]  {:5?}",
                    args[0], arg1, args[1], arg2, args[2]
                );
                args[2].set(machine, arg1 * arg2);
                (None, None)
            }
            Self::Read => {
                let readed = input.next().await.unwrap();
                #[cfg(feature = "debug")]
                println!("READ  [{:5}] {:5?}", readed, args[0]);
                args[0].set(machine, readed);
                (None, None)
            }
            Self::Write => {
                let writting = args[0].get(machine);
                #[cfg(feature = "debug")]
                println!("WRT   {:5?}[{:5}]", args[0], writting);
                (None, Some(writting))
            }
            Self::JmpT => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "JMPT  {:5?}[{:5}]  {:5?}[{:5}]",
                    args[0], arg1, args[1], arg2
                );
                let new_pc = if arg1 != 0 { Some(arg2 as usize) } else { None };
                (new_pc, None)
            }
            Self::JmpF => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "JMPF  {:5?}[{:5}]  {:5?}[{:5}]",
                    args[0], arg1, args[1], arg2
                );
                let new_pc = if arg1 == 0 { Some(arg2 as usize) } else { None };
                (new_pc, None)
            }
            Self::Less => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "LESS  {:5?}[{:5}]  {:5?}[{:5}]  {:5?}",
                    args[0], arg1, args[1], arg2, args[2]
                );
                if arg1 < arg2 {
                    args[2].set(machine, 1);
                } else {
                    args[2].set(machine, 0);
                }
                (None, None)
            }
            Self::Equal => {
                let arg1 = args[0].get(machine);
                let arg2 = args[1].get(machine);
                #[cfg(feature = "debug")]
                println!(
                    "EQ    {:5?}[{:5}]  {:5?}[{:5}]  {:5?}",
                    args[0], arg1, args[1], arg2, args[2]
                );
                if arg1 == arg2 {
                    args[2].set(machine, 1);
                } else {
                    args[2].set(machine, 0);
                }
                (None, None)
            }
            Self::MoveBase => {
                let arg = args[0].get(machine);
                #[cfg(feature = "debug")]
                println!("MVB   {:5?}[{:5}]", args[0], arg);
                machine.relative_base += arg as isize;
                (None, None)
            }
        }
    }

    fn args(&self) -> usize {
        match self {
            Self::Add | Self::Mul | Self::Less | Self::Equal => 3,
            Self::Read | Self::Write | Self::MoveBase => 1,
            Self::JmpT | Self::JmpF => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Imm(i128),
    Pos(usize),
    Rel(isize),
}

impl Argument {
    fn get(&self, machine: &mut Machine) -> i128 {
        let idx = match self {
            Self::Imm(v) => return *v,
            Self::Pos(a) => *a,
            Self::Rel(r) => (machine.relative_base + *r) as usize,
        };

        if idx >= machine.memory.len() {
            0
        } else {
            machine.memory[idx]
        }
    }

    fn set(&self, machine: &mut Machine, val: i128) {
        let idx = match self {
            Self::Imm(_) => panic!("Trying to output to immediate argument"),
            Self::Pos(a) => *a,
            Self::Rel(r) => (machine.relative_base + *r) as usize,
        };

        if idx >= machine.memory.len() {
            machine.memory.resize(idx + 1, 0);
        }

        machine.memory[idx] = val;
    }
}

// Returns (output_val, new_pc)
// For termination opcode, returned value should be `None`
// For non output opcodes, returned `output_val` should be None
async fn handle_opcode<S: Stream<Item = i128> + Unpin>(
    pc: usize,
    machine: &mut Machine,
    input: &mut S,
) -> Option<(Option<i128>, usize)> {
    let mut opcode = machine.memory[pc];
    #[cfg(feature = "debug")]
    print!("{:4}: [{:5}] ", pc, opcode);

    if opcode == 99 {
        #[cfg(feature = "debug")]
        println!("EXIT");
        return None;
    }

    let op = Op::new(opcode);
    opcode /= 100;

    let mut args = [Argument::Imm(0); 3];
    for (i, arg) in (0..op.args()).zip(args.iter_mut()) {
        let v = machine.memory[pc + i + 1];

        *arg = match opcode % 10 {
            0 => Argument::Pos(v as usize),
            1 => Argument::Imm(v),
            2 => Argument::Rel(v as isize),
            m => panic!("Invalid argument mode: {}", m),
        };

        opcode /= 10;
    }

    let (new_pc, output_val) = op.perform(&args, machine, input).await;
    let new_pc = new_pc.unwrap_or_else(|| pc + op.args() + 1);
    Some((output_val, new_pc))
}

pub fn interpret<S: Stream<Item = i128> + Unpin>(
    program: Vec<i128>,
    input: S,
) -> impl Stream<Item = i128> {
    stream!(
        let program = program;
        let mut input = input;
        let mut pc = 0;
        let mut machine = Machine {
            memory: program,
            relative_base: 0,
        };

        while let Some((output, new_pc)) = handle_opcode(pc, &mut machine, &mut input).await {
            yield output;
            pc = new_pc;
        }
    )
    .filter_map(std::convert::identity)
}

pub async fn parse_program<S: Stream<Item = String> + Unpin>(input: &mut S) -> Vec<i128> {
    input
        .next()
        .await
        .unwrap()
        .split(',')
        .map(str::parse::<i128>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

