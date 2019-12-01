use std::io::{stdin, BufRead, BufReader};
fn main() {
    let result: i64 = BufReader::new(stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| l.parse::<i64>().ok())
        .map(|l| l / 3 - 2)
        .sum();

    println!("{}", result)
}
