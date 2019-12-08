use async_std::prelude::*;
use async_std::stream::Stream;

async fn parse<S: Stream<Item = String> + Unpin>(mut input: S) -> Vec<u8> {
    input.next().await.unwrap().into_bytes()
}

// Returns (zeros, ones, twos)
fn count_digits(layer: &[u8]) -> (usize, usize, usize) {
    layer
        .iter()
        .fold((0, 0, 0), |(zeros, ones, twos), dig| match *dig {
            b'0' => (zeros + 1, ones, twos),
            b'1' => (zeros, ones + 1, twos),
            b'2' => (zeros, ones, twos + 1),
            _ => (zeros, ones, twos),
        })
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(input: S) -> usize {
    let img = parse(input).await;
    let (_, ones, twos) = img
        .chunks(25 * 6)
        .map(|chunk| count_digits(chunk))
        .min_by_key(|(zeros, _, _)| *zeros)
        .unwrap();
    ones * twos
}

fn flat_img(layers: &[&[u8]], len: usize) -> Vec<u8> {
    (0..len)
        .map(|px| {
            layers
                .iter()
                .map(|layer| layer[px])
                .find(|col| *col != b'2')
                .unwrap_or(b'2')
        })
        .collect()
}

fn make_line(line: &[u8]) -> String {
    line.iter()
        .map(|d| match d {
            b'0' => '■',
            b'1' => '□',
            _ => ' ',
        })
        .collect()
}

fn print_img(img: &[u8], width: usize) -> String {
    let img: Vec<_> = img.chunks(width).map(make_line).collect();
    img.join("\n")
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(input: S) -> String {
    let img = parse(input).await;
    let layers: Vec<_> = img.chunks(25 * 6).collect();
    let img = flat_img(&layers, 25 * 6);
    print_img(&img, 25)
}

