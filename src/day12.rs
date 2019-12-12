use async_std::prelude::*;
use async_std::stream::Stream;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, combinator::opt,
    sequence::tuple, IResult,
};
use std::borrow::Borrow;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Moon {
    fn update_vel(coord: i32, vel: i32, other: i32) -> i32 {
        if coord < other {
            vel + 1
        } else if coord > other {
            vel - 1
        } else {
            vel
        }
    }

    fn update(&mut self, others: impl Iterator<Item = impl Borrow<Moon>>) {
        for moon in others {
            let moon = moon.borrow();
            for i in 0..3 {
                self.velocity[i] =
                    Self::update_vel(self.position[i], self.velocity[i], moon.position[i]);
            }
        }

        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }

    fn energy(&self) -> i32 {
        self.position.iter().map(|c| c.abs()).sum::<i32>()
            * self.velocity.iter().map(|v| v.abs()).sum::<i32>()
    }
}

fn simulate(moons: [Moon; 4]) -> impl Iterator<Item = [Moon; 4]> {
    std::iter::successors(Some(moons), |prev| {
        let mut new = prev.clone();
        for (idx, moon) in new.iter_mut().enumerate() {
            let others = prev[..idx].iter().chain(prev[idx + 1..].iter());
            moon.update(others);
        }

        Some(new)
    })
}

fn number(input: &str) -> IResult<&str, i32> {
    map(
        tuple((opt(tag("-")), digit1)),
        |(sign, dig): (Option<&str>, &str)| {
            let res: i32 = dig.parse().unwrap();
            if sign.is_some() {
                -res
            } else {
                res
            }
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (i32, i32, i32)> {
    map(
        tuple((
            tag("<x="),
            number,
            tag(", y="),
            number,
            tag(", z="),
            number,
            tag(">"),
        )),
        |(_, x, _, y, _, z, _)| (x, y, z),
    )(input)
}

fn parse_moon(input: &str) -> Option<Moon> {
    let (_, moon): (&str, Moon) = map(parse_line, |(x, y, z)| Moon {
        position: [x, y, z],
        velocity: [0, 0, 0],
    })(input)
    .ok()?;

    Some(moon)
}

#[allow(unused)]
pub async fn simplified(input: impl Stream<Item = String>) -> i32 {
    let moons: Vec<_> = input.map(|line| parse_moon(&line).unwrap()).collect().await;
    let moons = [
        moons[0].clone(),
        moons[1].clone(),
        moons[2].clone(),
        moons[3].clone(),
    ];
    simulate(moons)
        .nth(1000)
        .unwrap()
        .iter()
        .map(|moon| moon.energy())
        .sum()
}

fn make_axis(moons: &[Moon; 4], axis: usize) -> [(i32, i32); 4] {
    [
        (moons[0].position[axis], moons[0].velocity[axis]),
        (moons[1].position[axis], moons[1].velocity[axis]),
        (moons[2].position[axis], moons[2].velocity[axis]),
        (moons[3].position[axis], moons[3].velocity[axis]),
    ]
}

fn get_period(moons: [Moon; 4], axis: usize) -> usize {
    let axises = make_axis(&moons, axis);
    simulate(moons.clone())
        .skip(1)
        .position(|state| axises == make_axis(&state, axis))
        .unwrap()
        + 1
}

fn gdc(mut a: usize, mut b: usize) -> usize {
    while a != 0 && b != 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    std::cmp::max(a, b)
}

fn lcm3(mut a: usize, mut b: usize, mut c: usize) -> usize {
    a /= gdc(a, b);
    b /= gdc(b, c);
    c /= gdc(c, a);
    a * b * c
}

#[allow(unused)]
pub async fn extended(input: impl Stream<Item = String>) -> usize {
    let moons: Vec<_> = input.map(|line| parse_moon(&line).unwrap()).collect().await;
    let moons = [
        moons[0].clone(),
        moons[1].clone(),
        moons[2].clone(),
        moons[3].clone(),
    ];
    let x_period = get_period(moons.clone(), 0);
    let y_period = get_period(moons.clone(), 1);
    let z_period = get_period(moons, 2);
    lcm3(x_period, y_period, z_period)
}
