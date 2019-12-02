use async_std;
use async_std::io::{stdin, BufReader};
use async_std::prelude::*;

mod day1;
#[cfg(feature = "day1")]
use day1 as day;

#[cfg(feature = "day2")]
mod day2;
#[cfg(feature = "day2")]
use day2 as day;

#[cfg(not(feature = "basic"))]
use day::extended as solution;
#[cfg(feature = "basic")]
use day::simplified as solution;

#[async_std::main]
async fn main() {
    let input = BufReader::new(stdin())
        .lines()
        .filter_map(|l| l.ok()?.parse().ok());
    println!("{}", solution(input).await)
}
