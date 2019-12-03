use async_std::prelude::*;
use async_std::stream::Stream;
use std::num::ParseIntError;
use std::str::FromStr;

enum Line {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

enum ParseLineErr {
    Empty,
    InvalidPrefix(char),
    ParseNumErr(ParseIntError),
}

impl FromStr for Line {
    type Err = ParseLineErr;

    fn from_str(s: &str) -> Result<Line, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }

        let (head, tail) = s.split_at(1);
        Ok(match head {
            "U" => Line::Up(tail.parse().map_err(ParseLineErr::ParseNumErr)?),
            "D" => Line::Down(tail.parse().map_err(ParseLineErr::ParseNumErr)?),
            "L" => Line::Left(tail.parse().map_err(ParseLineErr::ParseNumErr)?),
            "R" => Line::Right(tail.parse().map_err(ParseLineErr::ParseNumErr)?),
            _ => return Err(ParseLineErr::InvalidPrefix(head.chars().next().unwrap())),
        })
    }
}

impl Line {
    fn to_vec(self) -> (i64, i64) {
        match self {
            Self::Up(d) => (0, d),
            Self::Down(d) => (0, -d),
            Self::Left(d) => (-d, 0),
            Self::Right(d) => (d, 0),
        }
    }
}

mod simplified {
    use super::*;
    use std::cmp::{max, min};

    pub(super) fn parse_lines(input: &str) -> impl Iterator<Item = ((i64, i64), (i64, i64))> + '_ {
        let mut origin = (0, 0);
        input
            .split(',')
            .map(str::parse::<Line>)
            .filter_map(Result::ok)
            .map(Line::to_vec)
            .map(move |(x, y)| {
                let line = (origin, (origin.0 + x, origin.1 + y));
                origin = line.1;
                line
            })
            // Assumption - either x or y coords are equal,
            // so coords are swapped so lower coords are always on first point
            .map(|((x1, y1), (x2, y2))| ((min(x1, x2), min(y1, y2)), (max(x1, x2), max(y1, y2))))
    }

    // Returns distance between intersection and (0, 0) if lines intersects
    pub(super) fn intersect(
        first: &((i64, i64), (i64, i64)),
        second: &((i64, i64), (i64, i64)),
    ) -> Option<i64> {
        // Assumption 1: either `x` or `y` coords of each lines are equal
        // Assumption 2: lower coord is always on first point of line
        match ((first.0 .0 == first.1 .0), (second.0 .0 == second.1 .0)) {
            // In both lines `x` coords are qual
            (true, true) => {
                let y1 = max(first.0 .1, second.0 .1);
                let y2 = min(first.1 .1, second.1 .1);
                if first.0 .0 == second.0 .0 && y1 < y2 {
                    Some(min(y1.abs(), y2.abs()))
                } else {
                    None
                }
            }
            // In both lines 'y' coords are equal
            (false, false) => {
                let x1 = max(first.0 .0, second.0 .0);
                let x2 = min(first.1 .0, second.1 .0);
                if first.0 .1 == second.0 .1 && x1 < x2 {
                    Some(min(x1.abs(), x2.abs()))
                } else {
                    None
                }
            }
            // In first line `x` are equal, in second `y` are equal
            (true, false) => {
                if second.0 .0 <= first.0 .0
                    && first.0 .0 <= second.1 .0
                    && first.0 .1 <= second.0 .1
                    && second.0 .1 <= first.1 .1
                {
                    Some(first.0 .0.abs() + second.0 .1.abs())
                } else {
                    None
                }
            }
            // In first line `y` are equal, in second `x` are equal
            (false, true) => {
                if second.0 .1 <= first.0 .1
                    && first.0 .1 <= second.1 .1
                    && first.0 .0 <= second.0 .0
                    && second.0 .0 <= first.1 .0
                {
                    Some(second.0 .0.abs() + first.0 .1.abs())
                } else {
                    None
                }
            }
        }
    }
}

mod extended {
    use super::*;
    use std::cmp::{max, min};

    pub(super) fn parse_lines(
        input: &str,
    ) -> impl Iterator<Item = (i64, (i64, i64), (i64, i64))> + '_ {
        let mut origin = (0, 0);
        let mut steps = 0;
        input
            .split(',')
            .map(str::parse::<Line>)
            .filter_map(Result::ok)
            .map(Line::to_vec)
            .map(move |(x, y)| {
                let line = (steps, origin, (origin.0 + x, origin.1 + y));
                origin = line.2;
                steps += x.abs() + y.abs();
                line
            })
    }

    // Returns distance between intersection and (0, 0) if lines intersects
    pub(super) fn intersect(
        first_steps: i64,
        first: &((i64, i64), (i64, i64)),
        second_steps: i64,
        second: &((i64, i64), (i64, i64)),
    ) -> Option<i64> {
        let steps = first_steps + second_steps;

        // Assumption 1: either `x` or `y` coords of each lines are equal
        match ((first.0 .0 == first.1 .0), (second.0 .0 == second.1 .0)) {
            // In both lines `x` coords are qual
            (true, true) => {
                let y1 = max(min(first.0 .1, first.1 .1), min(second.0 .1, second.1 .1));
                let y2 = min(max(first.0 .1, first.1 .1), max(second.0 .1, second.1 .1));
                if first.0 .0 == second.0 .0 && y1 < y2 {
                    let y = if y1.abs() < y2.abs() { y1 } else { y2 };
                    Some(steps + (first.0 .1 - y).abs() + (second.0 .1 - y).abs())
                } else {
                    None
                }
            }
            // In both lines 'y' coords are equal
            (false, false) => {
                let x1 = max(min(first.0 .0, first.1 .0), min(second.0 .0, second.1 .0));
                let x2 = min(max(first.1 .0, first.1 .0), min(second.0 .0, second.1 .0));
                if first.0 .1 == second.0 .1 && x1 < x2 {
                    let x = if x1.abs() < x2.abs() { x1 } else { x2 };
                    Some(steps + (first.0 .0 - x).abs() + (second.0 .0 - x).abs())
                } else {
                    None
                }
            }
            // In first line `x` are equal, in second `y` are equal
            (true, false) => {
                let fymin = min(first.0 .1, first.1 .1);
                let fymax = max(first.0 .1, first.1 .1);
                let sxmin = min(second.0 .0, second.1 .0);
                let sxmax = max(second.0 .0, second.1 .0);
                let x = first.0 .0;
                let y = second.0 .1;
                if sxmin <= x && x <= sxmax && fymin <= y && y <= fymax {
                    Some(steps + (first.0 .1 - y).abs() + (second.0 .0 - x).abs())
                } else {
                    None
                }
            }
            // In first line `y` are equal, in second `x` are equal
            (false, true) => {
                let fxmin = min(first.0 .0, first.1 .0);
                let fxmax = max(first.0 .0, first.1 .0);
                let symin = min(second.0 .1, second.1 .1);
                let symax = max(second.0 .1, second.1 .1);
                let x = second.0 .0;
                let y = first.0 .1;

                if symin <= y && y <= symax && fxmin <= x && x <= fxmax {
                    Some(steps + (first.0 .0 - x).abs() + (second.0 .1 - y).abs())
                } else {
                    None
                }
            }
        }
    }
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> i64 {
    let wire: Vec<_> = simplified::parse_lines(&input.next().await.unwrap()).collect();

    simplified::parse_lines(&input.next().await.unwrap())
        .filter_map(|line| {
            wire.iter()
                .filter_map(|other_line| simplified::intersect(&line, other_line))
                .filter(|d| *d > 0)
                .min()
        })
        .min()
        .unwrap()
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(mut input: S) -> i64 {
    let wire: Vec<_> = extended::parse_lines(&input.next().await.unwrap()).collect();

    extended::parse_lines(&input.next().await.unwrap())
        .filter_map(|(s1, from1, to1)| {
            wire.iter()
                .filter_map(|(s2, from2, to2)| {
                    extended::intersect(s1, &(from1, to1), *s2, &(*from2, *to2))
                })
                .filter(|d| *d > 0)
                .min()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod simplified {
        use super::super::simplified::{intersect, parse_lines};

        #[test]
        fn parsing() {
            let input = "U1,R2,D3,L4";
            let res: Vec<_> = parse_lines(input).collect();
            let expected = vec![
                ((0, 0), (0, 1)),
                ((0, 1), (2, 1)),
                ((2, -2), (2, 1)),
                ((-2, -2), (2, -2)),
            ];

            assert_eq!(expected, res);
        }

        #[test]
        fn intersect_test() {
            assert_eq!(None, intersect(&((0, 0), (0, 1)), &((1, 2), (1, 3))));
            assert_eq!(Some(0), intersect(&((0, -1), (0, 1)), &((-1, 0), (1, 0))));
            assert_eq!(Some(5), intersect(&((2, -1), (2, 6)), &((1, 3), (5, 3))));
            assert_eq!(Some(0), intersect(&((-1, 0), (1, 0)), &((0, -1), (0, 1))));
            assert_eq!(Some(5), intersect(&((1, 3), (5, 3)), &((2, -1), (2, 6))));
            assert_eq!(None, intersect(&((0, 0), (0, 2)), &((1, 0), (1, 2))));
            assert_eq!(None, intersect(&((0, 0), (2, 0)), &((1, 1), (2, 1))));
        }
    }

    mod extended {
        use super::super::extended::{intersect, parse_lines};

        #[test]
        fn parsing() {
            let input = "U1,R2,D3,L4";
            let res: Vec<_> = parse_lines(input).collect();
            let expected = vec![
                (0, (0, 0), (0, 1)),
                (1, (0, 1), (2, 1)),
                (3, (2, 1), (2, -2)),
                (6, (2, -2), (-2, -2)),
            ];

            assert_eq!(expected, res);
        }

        #[test]
        fn intersect_test() {
            assert_eq!(None, intersect(0, &((0, 0), (0, 1)), 0, &((1, 2), (1, 3))));
            assert_eq!(
                Some(2),
                intersect(0, &((0, -1), (0, 1)), 0, &((-1, 0), (1, 0)))
            );
            assert_eq!(
                Some(5),
                intersect(0, &((2, -1), (2, 6)), 0, &((1, 3), (5, 3)))
            );
            assert_eq!(
                Some(2),
                intersect(0, &((-1, 0), (1, 0)), 0, &((0, -1), (0, 1)))
            );
            assert_eq!(
                Some(5),
                intersect(0, &((1, 3), (5, 3)), 0, &((2, -1), (2, 6)))
            );
            assert_eq!(None, intersect(0, &((0, 0), (0, 2)), 0, &((1, 0), (1, 2))));
            assert_eq!(None, intersect(0, &((0, 0), (2, 0)), 0, &((1, 1), (2, 1))));
        }
    }
}
