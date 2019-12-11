use crate::intcode::{interpret, parse_program};
use async_std::stream::Stream;
use async_std::sync::channel;
use async_stream::stream;
use futures::stream::StreamExt;
use futures_util::pin_mut;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self, dir: i128) -> Self {
        match (self, dir) {
            (Self::Up, 0) | (Self::Down, 1) => Self::Left,
            (Self::Right, 0) | (Self::Left, 1) => Self::Up,
            (Self::Down, 0) | (Self::Up, 1) => Self::Right,
            (Self::Left, 0) | (Self::Right, 1) => Self::Down,
            (_, d) => unimplemented!("Invalid direction code: {}", d),
        }
    }

    fn shift(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (x, y + 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y - 1),
            Self::Left => (x - 1, y),
        }
    }
}

struct PaintingRobot {
    visited: HashSet<(isize, isize)>,
    whites: HashSet<(isize, isize)>,
}

impl PaintingRobot {
    fn new() -> Self {
        Self {
            visited: HashSet::new(),
            whites: HashSet::new(),
        }
    }

    fn paint<'a, S: Stream<Item = i128> + Unpin + 'a>(
        &'a mut self,
        mut input: S,
    ) -> impl Stream<Item = i128> + 'a {
        stream!(
            let mut pos = (0, 0);
            let mut dir = Direction::Up;

            loop {
                yield if self.whites.contains(&pos) { 1 } else { 0 };

                match input.next().await {
                    None => break,
                    Some(0) => { self.whites.remove(&pos); },
                    Some(1) => { self.whites.insert(pos.clone()); },
                    Some(c) => unreachable!("Invalid color code: {}", c),
                };
                self.visited.insert(pos.clone());

                dir = dir.rotate(input.next().await.unwrap());
                pos = dir.shift(pos);
            }
        )
    }
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> usize {
    let program = parse_program(&mut input).await;
    let mut robot = PaintingRobot::new();
    let (send, recv) = channel(1);

    let program_output = interpret(program, recv);
    pin_mut!(program_output);
    robot
        .paint(program_output)
        .for_each(|color| send.send(color))
        .await;

    robot.visited.len()
}

fn draw(image: &HashSet<(isize, isize)>) -> String {
    let (minx, _) = image.iter().min_by_key(|(x, _)| *x).unwrap();
    let (_, miny) = image.iter().min_by_key(|(_, y)| *y).unwrap();
    let (maxx, _) = image.iter().max_by_key(|(x, _)| *x).unwrap();
    let (_, maxy) = image.iter().max_by_key(|(_, y)| *y).unwrap();

    let lines: Vec<_> = (minx - 1..=maxx + 1)
        .map(|x| {
            (miny - 1..=maxy + 1)
                .map(|y| if image.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect();
    lines.join("\n")
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(mut input: S) -> () {
    let program = parse_program(&mut input).await;
    let mut robot = PaintingRobot::new();
    robot.whites.insert((0, 0));
    let (send, recv) = channel(1);

    let program_output = interpret(program, recv);
    pin_mut!(program_output);
    robot
        .paint(program_output)
        .for_each(|color| send.send(color))
        .await;

    println!("{}", draw(&robot.whites));
}

