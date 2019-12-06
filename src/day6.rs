use async_std::prelude::*;
use async_std::stream::Stream;
use std::collections::HashMap;
use std::iter::successors;

async fn parse_input(input: impl Stream<Item = String>) -> HashMap<String, String> {
    input
        .filter_map(|s| {
            let mut it = s.split(')');
            let center = it.next()?;
            let orbit = it.next()?;
            Some((orbit.to_owned(), center.to_owned()))
        })
        .collect()
        .await
}

fn parents<'m, 'a: 'm>(
    s: &'a str,
    map: &'m HashMap<String, String>,
) -> impl Iterator<Item = &'m str> + 'm {
    successors(Some(s), move |s| map.get(*s).map(String::as_str)).skip(1)
}

#[allow(unused)]
pub async fn simplified(input: impl Stream<Item = String>) -> usize {
    let graph = parse_input(input).await;
    let mut counters = HashMap::with_capacity(graph.len());
    for planet in graph.keys() {
        for parent in parents(&planet, &graph) {
            counters
                .entry(parent)
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
        }
    }
    counters.values().sum()
}

#[allow(unused)]
pub async fn extended(input: impl Stream<Item = String>) -> usize {
    let graph = parse_input(input).await;
    let mut my_parents: Vec<_> = parents("YOU", &graph).enumerate().collect();
    my_parents.sort_by_key(|(_, p)| *p);

    let mut dists = parents("SAN", &graph)
        .enumerate()
        .filter_map(|(dist, parent)| {
            my_parents
                .binary_search_by_key(&parent, |(_, p)| p)
                .ok()
                .map(|idx| my_parents[idx].0 + dist)
        });

    dists.next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{extended, simplified};
    use async_std;
    use async_std::stream::from_iter;

    #[async_std::test]
    async fn simplified_test() -> std::io::Result<()> {
        let given = simplified(from_iter(
            vec![
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
            ]
            .into_iter()
            .map(str::to_owned),
        ))
        .await;

        assert_eq!(42, given);
        Ok(())
    }

    #[async_std::test]
    async fn extended_test() -> std::io::Result<()> {
        let given = extended(from_iter(
            vec![
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
                "K)YOU", "I)SAN",
            ]
            .into_iter()
            .map(str::to_owned),
        ))
        .await;

        assert_eq!(4, given);
        Ok(())
    }
}
