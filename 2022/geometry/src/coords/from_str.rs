use crate::coords::from_str::ParseCoordsError::InvalidCoordCount;
use crate::coords::Coords;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

impl<const N: usize> FromStr for Coords<N> {
    type Err = ParseCoordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<_> = s
            .split(',')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();

        if input.len() != N {
            return Err(InvalidCoordCount {
                expected: N,
                found: input.len(),
            });
        }

        let mut coords = [0; N];
        coords[..N].copy_from_slice(&input[..N]);

        Ok(Coords(coords))
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseCoordsError {
    #[error("invalid number of coordinates (expected {expected}, found {found})")]
    InvalidCoordCount { expected: usize, found: usize },

    #[error("parsing coordinate failed")]
    InvalidCoord(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("42".parse(), Ok(Coords([42])));
        assert_eq!("-10,10".parse(), Ok(Coords([-10, 10])));
        assert_eq!(
            "1,1,1".parse::<Coords<2>>(),
            Err(InvalidCoordCount {
                expected: 2,
                found: 3
            })
        )
    }
}
