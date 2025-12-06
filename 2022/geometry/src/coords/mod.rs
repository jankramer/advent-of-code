mod adjacent;
mod arithmetic;
mod display;
mod distance;
mod from_str;
mod neighbours;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Coords<const N: usize>(pub [isize; N]);
