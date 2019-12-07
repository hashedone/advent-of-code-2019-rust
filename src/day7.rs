use async_std;
use async_std::future::ready;
use async_std::pin::Pin;
use async_std::prelude::*;
use async_std::stream::{self, Stream};
use async_std::sync::channel;
use futures::future::join_all;
use futures::stream::StreamExt;
use permute::permutations_of;

use crate::intcode::interpret;

async fn parse_program<S: Stream<Item = String> + Unpin>(input: &mut S) -> Vec<i64> {
    input
        .next()
        .await
        .unwrap()
        .split(',')
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> i64 {
    let program = parse_program(&mut input).await;

    let results = permutations_of(&[0, 1, 2, 3, 4]).map(|phases| {
        let init: Pin<Box<dyn Stream<Item = i64>>> = Box::pin(stream::once(0));

        let mut output = phases.fold(init, |input, phase| {
            let input = stream::once(*phase).chain(input);
            Box::pin(interpret(program.clone(), input))
        });

        async move { output.next().await }
    });

    join_all(results)
        .await
        .into_iter()
        .map(Option::unwrap)
        .max()
        .unwrap()
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(mut input: S) -> i64 {
    let program = parse_program(&mut input).await;

    let results = permutations_of(&[5, 6, 7, 8, 9]).map(|phases| {
        let (send, recv) = channel(1);
        let init: Pin<Box<dyn Stream<Item = _>>> = Box::pin(stream::once(0).chain(recv));
        let program = &program;

        let init_fut: Pin<Box<dyn Future<Output = _>>> = Box::pin(ready(0));
        let mut last_outputs = phases.fold(init, move |input, phase| {
            let input = stream::once(*phase).chain(input);
            Box::pin(interpret(program.clone(), input))
        });

        last_outputs.fold(0, move |_, output| {
            let send = send.clone();
            async move {
                send.clone().send(output).await;
                output
            }
        })
    });

    join_all(results).await.into_iter().max().unwrap()
}
