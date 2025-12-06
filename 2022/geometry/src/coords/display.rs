use crate::coords::Coords;
use std::fmt::{Display, Formatter};

impl<const N: usize> Display for Coords<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map(|x| x.to_string()).join(","))
    }
}
