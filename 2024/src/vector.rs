use std::ops::{Add, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vec2(pub i64, pub i64);

impl Vec2 {
    pub fn nb4(&self) -> impl Iterator<Item = Vec2> + '_ {
        [Vec2(-1, 0), Vec2(0, 1), Vec2(1, 0), Vec2(0, -1)]
            .iter()
            .map(move |h| self + h)
    }

    pub fn nb8(&self) -> impl Iterator<Item = Vec2> + '_ {
        [
            Vec2(-1, 0),
            Vec2(-1, 1),
            Vec2(0, 1),
            Vec2(1, 1),
            Vec2(1, 0),
            Vec2(1, -1),
            Vec2(0, -1),
            Vec2(-1, -1),
        ]
        .iter()
        .map(move |h| self + h)
    }

    pub fn nb4_diag(&self) -> impl Iterator<Item = Vec2> + '_ {
        [Vec2(-1, 1), Vec2(1, 1), Vec2(1, -1), Vec2(-1, -1)]
            .iter()
            .map(move |h| self + h)
    }

    pub fn taxicab(&self, rhs: &Vec2) -> i64 {
        (rhs.0 - self.0).abs() + (rhs.1 - self.1).abs()
    }
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Vec2(x, y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Vec2> for i64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self as i64 * rhs.0, self as i64 * rhs.1)
    }
}

impl Rem for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: Vec2) -> Vec2 {
        Self(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

impl Rem<&Vec2> for Vec2 {
    type Output = Vec2;

    fn rem(self, rhs: &Vec2) -> Vec2 {
        Self(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StateVec {
    pub r: Vec2,
    pub v: Vec2,
}

impl StateVec {
    pub fn new(r: Vec2, v: Vec2) -> Self {
        Self { r, v }
    }

    pub fn at_time(&self, t: i64) -> Vec2 {
        self.r + t * self.v
    }

    pub fn step(&self, t: i64) -> Self {
        Self::new(self.at_time(t), self.v)
    }

    pub fn rotate_left(&self) -> Self {
        Self::new(self.r, Vec2(-self.v.1, self.v.0))
    }

    pub fn rotate_right(&self) -> Self {
        Self::new(self.r, Vec2(self.v.1, -self.v.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let a = Vec2(4, 2);
        let b = Vec2(1, 3);

        assert_eq!(a + b, Vec2(5, 5));
        assert_eq!(a - b, Vec2(3, -1));
        assert_eq!(3 * b, Vec2(3, 9));
        assert_eq!((Vec2(0, 0) - Vec2(1, 1)) % Vec2(4, 4), Vec2(3, 3));
        assert_eq!(Vec2(5, 5) % Vec2(4, 4), Vec2(1, 1));
    }

    #[test]
    fn neighbors() {
        assert_eq!(
            Vec2(5, 5).nb4().collect::<Vec<_>>(),
            vec![Vec2(4, 5), Vec2(5, 6), Vec2(6, 5), Vec2(5, 4)]
        );

        assert_eq!(
            Vec2(5, 5).nb4_diag().collect::<Vec<_>>(),
            vec![Vec2(4, 6), Vec2(6, 6), Vec2(6, 4), Vec2(4, 4)]
        );

        assert_eq!(
            Vec2(5, 5).nb8().collect::<Vec<_>>(),
            vec![
                Vec2(4, 5),
                Vec2(4, 6),
                Vec2(5, 6),
                Vec2(6, 6),
                Vec2(6, 5),
                Vec2(6, 4),
                Vec2(5, 4),
                Vec2(4, 4)
            ]
        );
    }

    #[test]
    fn state_vec() {
        let vec = StateVec::new(Vec2(0, 0), Vec2(1, 1));
        let actual: Vec<_> = (0..3).map(|t| vec.at_time(t)).collect();

        assert_eq!(actual, vec![Vec2(0, 0), Vec2(1, 1), Vec2(2, 2)]);
    }
}
