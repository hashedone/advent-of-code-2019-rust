use crate::intcode::{interpret, parse_program};
use async_std::prelude::*;
use async_std::stream::{self, Stream};
use async_stream::stream;
use futures_util::pin_mut;
#[cfg(feature = "visual")]
use pancurses::{endwin, initscr};
use std::sync::Mutex;
#[cfg(feature = "visual")]
use std::thread::sleep;
#[cfg(feature = "visual")]
use std::time::Duration;

fn sprites<S: Stream<Item = i128> + Unpin>(mut input: S) -> impl Stream<Item = (i128, i128, i128)> {
    stream! {
        while let Some(x) = input.next().await {
            yield(x, input.next().await.unwrap(), input.next().await.unwrap())
        }
    }
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(mut input: S) -> usize {
    let program = parse_program(&mut input).await;
    let output = interpret(program, stream::empty());
    pin_mut!(output);
    let sprites = sprites(output);
    pin_mut!(sprites);
    sprites.filter(|(_, _, id)| *id == 2).count().await
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(mut input: S) -> i128 {
    let mut program = parse_program(&mut input).await;
    program[0] = 2;

    let status = Mutex::new((0, 0));
    let control = stream::from_fn(|| {
        let (paddle_x, ball_x) = *status.lock().unwrap();
        if paddle_x < ball_x {
            Some(1)
        } else if paddle_x > ball_x {
            Some(-1)
        } else {
            Some(0)
        }
    });

    let output = interpret(program, control);
    pin_mut!(output);
    let sprites = sprites(output);
    pin_mut!(sprites);

    #[cfg(feature = "visual")]
    let window = initscr();
    let mut score = 0;

    #[cfg(feature = "visual")]
    while let Some((x, y, id)) = sprites.next().await {
        if x == -1 && y == 0 {
            window.mv(30, 0);
            window.addstr(format!("Score: {}", id));
            score = id;
        } else {
            window.mv(y as i32, x as i32);
            let c = match id {
                0 => ' ',
                1 => '#',
                2 => '$',
                3 => '_',
                4 => '*',
                _ => continue,
            };
            window.addch(c);

            if c == '*' {
                status.lock().unwrap().1 = x;
                sleep(Duration::from_millis(10));
            } else if c == '_' {
                status.lock().unwrap().0 = x;
            }
        }

        window.refresh();
    }

    #[cfg(not(feature = "visual"))]
    while let Some((x, y, id)) = sprites.next().await {
        if x == -1 && y == 0 {
            score = id;
        } else if id == 4 {
            status.lock().unwrap().1 = x;
        } else if id == 3 {
            status.lock().unwrap().0 = x;
        }
    }

    #[cfg(feature = "visual")]
    {
        sleep(Duration::from_secs(5));
        endwin();
    }

    score
}
