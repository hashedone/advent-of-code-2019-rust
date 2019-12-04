use async_std::prelude::*;
use async_std::stream::Stream;

async fn parse_input<S: Stream<Item = String> + Unpin>(mut input: S) -> (i64, i64) {
    let items = input
        .next()
        .await
        .unwrap()
        .split('-')
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    (items[0], items[1])
}

fn has_adj(s: &str) -> bool {
    let chars = s.chars();
    let chars2 = s.chars().skip(1); // offsetted by 1

    chars.zip(chars2).any(|(c1, c2)| c1 == c2)
}

fn has_adj_ex(s: &str) -> bool {
    let s = s.as_bytes();

    (0..s.len() - 3).any(|i| s[i + 1] == s[i + 2] && s[i] != s[i + 1] && s[i + 2] != s[i + 3])
        || (s[0] == s[1] && s[1] != s[2])
        || (s[s.len() - 1] == s[s.len() - 2] && s[s.len() - 2] != s[s.len() - 3])
}

fn is_increasing(s: &str) -> bool {
    let chars = s.chars();
    let chars2 = s.chars().skip(1); // offsetted by 1

    chars.zip(chars2).all(|(c1, c2)| c1 <= c2)
}

#[allow(unused)]
pub async fn simplified<S: Stream<Item = String> + Unpin>(input: S) -> usize {
    let (from, to) = parse_input(input).await;

    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| has_adj(&s))
        .filter(|s| is_increasing(&s))
        .count()
}

#[allow(unused)]
pub async fn extended<S: Stream<Item = String> + Unpin>(input: S) -> usize {
    let (from, to) = parse_input(input).await;

    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| has_adj_ex(&s))
        .filter(|s| is_increasing(&s))
        .count()
}

#[cfg(test)]
mod tests {
    use super::has_adj_ex;
    use super::parse_input;
    use async_std;
    use async_std::stream::from_iter;

    #[async_std::test]
    async fn parse() {
        assert_eq!(
            (10, 200),
            parse_input(from_iter(vec!["10-200".to_owned()])).await
        );
    }

    #[test]
    fn adj() {
        assert_eq!(true, has_adj_ex("112233"));
        assert_eq!(false, has_adj_ex("123444"));
        assert_eq!(true, has_adj_ex("111122"));
        assert_eq!(true, has_adj_ex("112222"));
        assert_eq!(true, has_adj_ex("123345"));
    }
}
