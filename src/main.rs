use async_std;
use async_std::io::{stdin, BufReader};
use async_std::prelude::*;
use tokio;

mod intcode;

mod day1;
#[cfg(feature = "day1")]
use day1 as day;

mod day2;
#[cfg(feature = "day2")]
use day2 as day;

mod day3;
#[cfg(feature = "day3")]
use day3 as day;

mod day4;
#[cfg(feature = "day4")]
use day4 as day;

mod day5;
#[cfg(feature = "day5")]
use day5 as day;

mod day6;
#[cfg(feature = "day6")]
use day6 as day;

mod day7;
#[cfg(feature = "day7")]
use day7 as day;

#[cfg(not(feature = "basic"))]
use day::extended as solution;
#[cfg(feature = "basic")]
use day::simplified as solution;

#[tokio::main]
async fn main() {
    let input = BufReader::new(stdin())
        .lines()
        .filter_map(|l| l.ok()?.parse().ok());
    println!("{:?}", solution(input).await)
}
