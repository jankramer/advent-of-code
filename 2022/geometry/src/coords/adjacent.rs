use crate::coords::Coords;

impl<const N: usize> Coords<N> {
    pub fn adjacent(self) -> impl Iterator<Item = Coords<N>> {
        (0..N).flat_map(move |n| {
            let mut left = [0; N];
            let mut right = [0; N];

            left[n] = -1;
            right[n] = 1;

            [self + Coords(left), self + Coords(right)]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        assert_eq!(
            Coords([1]).adjacent().collect::<Vec<_>>(),
            vec![Coords([0]), Coords([2])]
        );

        assert_eq!(
            Coords([1, 1]).adjacent().collect::<Vec<_>>(),
            vec![
                Coords([0, 1]),
                Coords([2, 1]),
                Coords([1, 0]),
                Coords([1, 2])
            ]
        );

        assert_eq!(
            Coords([1, 1, 1]).adjacent().collect::<Vec<_>>(),
            vec![
                Coords([0, 1, 1]),
                Coords([2, 1, 1]),
                Coords([1, 0, 1]),
                Coords([1, 2, 1]),
                Coords([1, 1, 0]),
                Coords([1, 1, 2]),
            ]
        );
    }
}
