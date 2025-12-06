use std::ops::{Add, Mul};

use crate::coords::Coords;

impl<const N: usize> Add<Coords<N>> for Coords<N> {
    type Output = Coords<N>;

    fn add(self, rhs: Coords<N>) -> Self::Output {
        let mut copy = self;
        for i in 0..N {
            copy.0[i] += rhs.0[i];
        }

        copy
    }
}

impl<const N: usize> Mul<Coords<N>> for isize {
    type Output = Coords<N>;

    fn mul(self, rhs: Coords<N>) -> Self::Output {
        let mut copy = rhs;
        for i in 0..N {
            copy.0[i] *= self;
        }

        copy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Coords([3, 1]);
        let b = Coords([1, 1]);

        assert_eq!(Coords([4, 2]), a + b);
        assert_eq!(Coords([3, 1]), a);
        assert_eq!(Coords([1, 1]), b);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(3 * Coords([5, 6]), Coords([15, 18]));
    }
}
