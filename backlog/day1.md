# Day 1: The Tyranny of the Rocket Equation, Part 1

Part one of excercise is actually trivial. What needs to be done,
is to read input line by line, calculate `input / 3 - 2` (input is
some rocket mass, and the result is fuel needed for travel for this
rocket, and sum everything up. All of this can be done easly using
iterators:

```rust
pub async fn solution(input: impl Stream<Item = i64>) -> i64 {
    input.map(|mass| mass / 3 - 2).sum().await
}
```

# Day 1: The Tyranny of the Rocket Equation, Part 2

After applying the result, second part of excercise got revealed.
This time elves found out, that rocket fuel also has own mass, so after
fuel calculation, there need to be another round - this time calculation
of additional fuel needed to carry the previously calculated fuel batch,
and the process needs to be repeated, unless additional fuel needed for
carrying the rocket is 0.

First what I decided to do is to create iterator, where every item is
fuel needed to carry on mass of previous item, like:
`[rocket-mass, fuel-for-0, fuel-for-1, ..]`.

Such an iterator would be infinite, but it is easly visible, that those
values are strictly decreasing and will eventually achieve 0.

Creating such an iterator is easy with function [`std::iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html):

```rust
successors(Some(mass), |mass| Some(*mass / 3 - 2)).skip(1)
```

I'm skipping the first element, because this would be the initial mass
of the rocket, which is not interesting for me.

Next think to do is to limit the infinite iterator, to stop when first `0`
is found, and sum needed fuel:

```rust
successors(Some(mass), |mass| Some(*mass / 3 - 2))
    .skip(1)
    .take_while(|fuel| *fuel > 0)
    .sum()
```

What left is an easy part - calculation need to be done for every input
line, and then just sum every partial result:

```rust
pub async fn extended(input: impl Stream<Item = i64>) -> i64 {
    let partials = |mass| {
        successors(Some(mass), |mass| Some(*mass / 3 - 2))
            .skip(1)
            .take_while(|fuel| *fuel > 0)
            .sum()
    };

    input.map(partials).sum().await
}
```
