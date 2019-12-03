use async_std::prelude::*;
use async_std::stream::{from_iter, Stream};

fn interpret(mut program: Vec<i64>) -> i64 {
    let mut idx = 0;

    while let Some(op) = program.get(idx) {
        let res = match op {
            1 => program[program[idx + 1] as usize] + program[program[idx + 2] as usize],
            2 => program[program[idx + 1] as usize] * program[program[idx + 2] as usize],
            99 => break,
            _ => unreachable!(),
        };
        let res_idx = program[idx + 3] as usize;
        program[res_idx] = res;

        idx += 4
    }

    program[0]
}

async fn parse_program(input: impl Stream<Item = String>) -> Vec<i64> {
    input
        .map(|l| {
            from_iter(
                l.split(',')
                    .map(str::parse::<i64>)
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect()
        .await
}

#[allow(unused)]
pub async fn simplified(input: impl Stream<Item = String>) -> i64 {
    let mut program = parse_program(input).await;

    program[1] = 12;
    program[2] = 2;

    interpret(program)
}

#[allow(unused)]
pub async fn extended(input: impl Stream<Item = String>) -> i64 {
    let program = parse_program(input).await;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = program.clone();

            program[1] = noun;
            program[2] = verb;

            if interpret(program) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::interpret;

    #[test]
    fn interpret_test() {
        assert_eq!(
            3500,
            interpret(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])
        );
        assert_eq!(2, interpret(vec![1, 0, 0, 0, 99]));
        assert_eq!(2, interpret(vec![2, 3, 0, 3, 99]));
        assert_eq!(2, interpret(vec![2, 4, 4, 5, 99, 0]));
        assert_eq!(30, interpret(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }
}
