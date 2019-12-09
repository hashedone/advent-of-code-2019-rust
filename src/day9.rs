use crate::intcode::interpret;
use async_std;
use async_std::stream::{self, Stream};
use futures::stream::StreamExt;

async fn parse_program<S: Stream<Item = String> + Unpin>(input: &mut S) -> Vec<i128> {
    input
        .next()
        .await
        .unwrap()
        .split(',')
        .map(str::parse::<i128>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> Vec<i128> {
    let program = parse_program(&mut input).await;
    interpret(program, stream::once(1)).collect().await
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(mut input: S) -> Vec<i128> {
    let program = parse_program(&mut input).await;
    interpret(program, stream::once(2)).collect().await
}

