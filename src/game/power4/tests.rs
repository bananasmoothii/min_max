#[cfg(test)]
mod iterators_test {
    use crate::game::power4::Power4;

    #[test]
    fn lines_passing_at() {
        let power4 = Power4::new();
        let lines = power4.lines_passing_at_longer_4((0, 0));
        assert_eq!(lines.len(), 3);
        let lines = power4.lines_passing_at_longer_4((2, 3));
        assert_eq!(
            lines
                .iter()
                .map(|iter| (iter.x, iter.y))
                .collect::<Vec<_>>(),
            vec![(0, 2), (3, 0), (1, 0), (0, 5)]
        );
    }
}
