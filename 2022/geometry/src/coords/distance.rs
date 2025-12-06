use crate::coords::Coords;
use std::borrow::Borrow;
use std::iter::zip;

#[allow(dead_code)]
pub fn manhattan_distance<const N: usize, T: Borrow<Coords<N>>>(a: T, b: T) -> usize {
    zip(a.borrow().0, b.borrow().0)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(1, manhattan_distance(Coords([0]), Coords([1])));
        assert_eq!(4, manhattan_distance(Coords([0, 0]), Coords([2, 2])));
        assert_eq!(9, manhattan_distance(Coords([0, 0, 0]), Coords([3, 3, 3])));
    }
}
