use async_std::prelude::*;
use async_std::stream::Stream;
use std::collections::HashSet;

enum Field {
    Empty,
    Asteroid,
}

struct Map {
    map: Vec<Field>,
    width: usize,
}

impl Map {
    fn coords(&self, idx: usize) -> (isize, isize) {
        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;
        (x, y)
    }
}

async fn parse(input: impl Stream<Item = String>) -> Map {
    let map: Vec<_> = input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Field::Empty,
                    '#' => Field::Asteroid,
                    _ => unreachable!("Invalid character on map!"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
        .await;

    let width = map[0].len();
    let map = map.into_iter().map(|l| l.into_iter()).flatten().collect();

    Map { map, width }
}

fn gcd(mut x: isize, mut y: isize) -> isize {
    while x != 0 && y != 0 {
        if x > y {
            x %= y;
        } else {
            y %= x;
        }
    }

    std::cmp::max(x, y)
}

fn count_visible((x, y): (isize, isize), map: &Map) -> usize {
    // Indices of hash map are "directions" - .0/.1 is tan of direction vector,
    // sgn(.0) determines direction on `x` axis, and sgn(.1) is direction on `y`
    // axis. Important thing is that .0/.1 should be the simplest (unified)
    // fraction
    //
    // This way the count of visible items is just a number of such entries -
    // in any direction there is at least one asteroid (because it is added),
    // and also every direction is counted once (because they are collected
    // into hash_set)

    map.map
        .iter()
        .enumerate()
        // Mapping asteroids to (x, y), where [x, y] is simplest direction vector
        .filter_map(|(idx, item)| match item {
            Field::Empty => None,
            Field::Asteroid => {
                let (xp, yp) = map.coords(idx);
                if x == xp && y == yp {
                    None
                } else {
                    let (x, y) = (xp - x, yp - y);
                    if x == 0 {
                        Some((0, y.signum()))
                    } else if y == 0 {
                        Some((x.signum(), 0))
                    } else {
                        let d = gcd(x.abs(), y.abs());
                        Some((x / d, y / d))
                    }
                }
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

fn transform_asteroids((x, y): (isize, isize), map: &Map) -> Vec<(isize, isize, isize)> {
    // Finds all asteroids and transformates them to form (x, y, dist)
    // where (x, y) is the simplest vector, and dist is distance to origin

    map.map
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| match item {
            Field::Empty => None,
            Field::Asteroid => {
                let (xp, yp) = map.coords(idx);
                if x == xp && y == yp {
                    None
                } else {
                    let (x, y) = (xp - x, yp - y);
                    if x == 0 {
                        Some((0, y.signum(), y.abs()))
                    } else if y == 0 {
                        Some((x.signum(), 0, x.abs()))
                    } else {
                        let d = gcd(x.abs(), y.abs());
                        Some((x / d, y / d, d))
                    }
                }
            }
        })
        .collect()
}

#[allow(unused)]
pub async fn simplified(input: impl Stream<Item = String>) -> usize {
    let map = parse(input).await;

    map.map
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| match item {
            Field::Empty => None,
            Field::Asteroid => Some(count_visible(map.coords(idx), &map)),
        })
        .max()
        .unwrap()
}

fn angle((x, y): (isize, isize)) -> f64 {
    -(x as f64).atan2(y as f64)
}

fn cmp_asteroids(
    left: &(isize, isize, isize),
    right: &(isize, isize, isize),
) -> std::cmp::Ordering {
    if left.0 == right.0 && left.1 == right.1 {
        left.2.cmp(&right.2)
    } else {
        angle((left.0, left.1))
            .partial_cmp(&angle((right.0, right.1)))
            .unwrap()
    }
}

#[allow(unused)]
pub async fn extended(input: impl Stream<Item = String>) -> isize {
    let map = parse(input).await;

    let (best, cnt) = map
        .map
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| match item {
            Field::Empty => None,
            Field::Asteroid => Some((idx, count_visible(map.coords(idx), &map))),
        })
        .max_by_key(|(_, cnt)| *cnt)
        .unwrap();

    let mut asteroids = transform_asteroids(map.coords(best), &map);
    asteroids.sort_by(cmp_asteroids);
    let mut asteroids: Vec<_> = asteroids.into_iter().map(Option::Some).collect();

    let first = asteroids[0].take();
    let mut ordered = std::iter::successors(first, move |(x, y, d)| {
        let next = asteroids
            .iter()
            .enumerate()
            .filter_map(|(idx, a)| a.map(|a| (idx, a)))
            .filter(|(_, (xp, yp, _))| x != xp || y != yp)
            .find(|(_, r)| cmp_asteroids(&(*x, *y, *d), r) == std::cmp::Ordering::Less)
            .map(|(idx, _)| idx);

        let next = next.or_else(|| {
            asteroids
                .iter()
                .enumerate()
                .filter_map(|(idx, a)| a.map(|_| idx))
                .next()
        });

        next.and_then(|next| asteroids[next].take())
    });

    let (x, y, _) = ordered.nth(199).unwrap();
    let (ox, oy) = map.coords(best);
    let (x, y) = (x + ox, y + oy);
    x * 100 + y
}

#[cfg(test)]
mod tests {
    use super::simplified;
    use async_std;
    use async_std::stream::from_iter;

    #[async_std::test]
    async fn simplified_test() -> std::io::Result<()> {
        assert_eq!(
            8,
            simplified(from_iter(vec![
                ".#..#".to_owned(),
                ".....".to_owned(),
                "#####".to_owned(),
                "....#".to_owned(),
                "...##".to_owned(),
            ]))
            .await
        );

        Ok(())
    }
}
