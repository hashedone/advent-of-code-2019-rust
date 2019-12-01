This year I decided to contribute in [Advent of Code](https://adventofcode.com/) trying myself in two languages: Rust which is for now my main language, and Haskell, which I try to learn this way (find my Haskell solutions [here](https://github.com/hashedone/advent-of-code-2019-hask)).

For Rust solutions I decided to:
* solve them using async/await for IO (just to check how they work for such cases);
* in as most "functional" way I can.

I also decided to document my progress of Rust solutions way of think.

# Rust framework

To make this simple I created simple framework in my `main.rs`, so every solution should have entry point signature:

```
async fn solution(
    input: impl async_std::stream::Stream<Input=impl std::str::FromStr>
) -> impl std::fmt::Display;
```

Input data would be parsed line-by-line, and output data would be just printed out.

# Solutions

* [Day1](https://github.com/hashedone/advent-of-code-2019-rust/backlog/day1.md)
