use async_std::prelude::*;
use async_std::stream::Stream;
use std::iter::successors;

fn for_mass(mass: i64) -> i64 {
    mass / 3 - 2
}

#[allow(unused)]
pub async fn simplified(input: impl Stream<Item = i64>) -> i64 {
    input.map(for_mass).sum().await
}

#[allow(unused)]
pub async fn extended(input: impl Stream<Item = i64>) -> i64 {
    let partials = |mass| {
        successors(Some(mass), |mass| Some(for_mass(*mass)))
            .skip(1)
            .take_while(|fuel| *fuel > 0)
            .sum()
    };

    input.map(partials).sum().await
}

#[cfg(test)]
mod test {
    use super::{extended, simplified};
    use async_std;
    use async_std::stream::from_iter;

    #[async_std::test]
    async fn simplified_test() -> std::io::Result<()> {
        assert_eq!(2, simplified(from_iter(vec![12])).await);
        assert_eq!(2, simplified(from_iter(vec![14])).await);
        assert_eq!(654, simplified(from_iter(vec![1969])).await);
        assert_eq!(33_583, simplified(from_iter(vec![100_756])).await);

        Ok(())
    }

    #[async_std::test]
    async fn extended_test() -> std::io::Result<()> {
        assert_eq!(2, extended(from_iter(vec![14])).await);
        assert_eq!(966, extended(from_iter(vec![1969])).await);
        assert_eq!(50_346, extended(from_iter(vec![100_756])).await);

        Ok(())
    }
}
