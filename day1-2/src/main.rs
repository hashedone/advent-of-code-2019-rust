use std::io::{stdin, BufRead, BufReader};
use std::iter;

fn calc_fuel(mass: i64) -> i64 {
    iter::successors(Some(mass), |fuel| {
        let need = fuel / 3 - 2;
        if need > 0 {
            Some(need)
        } else {
            None
        }
    })
    .skip(1)
    .sum()
}

fn main() {
    let result: i64 = BufReader::new(stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| l.parse::<i64>().ok())
        .map(calc_fuel)
        .sum();

    println!("{}", result)
}
