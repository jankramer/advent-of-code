use crate::grid::Grid;
use crate::vector::Vec2;
use itertools::Itertools;
use regex::Regex;
use std::ops::Deref;
use std::sync::LazyLock;

pub struct Input<'a>(&'a str);

static NUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+").unwrap());

impl<'a> Input<'a> {
    pub const fn new(input: &'a str) -> Input<'a> {
        Self(input)
    }

    pub fn numbers(&self) -> impl Iterator<Item = i64> + '_ {
        NUM_REGEX
            .find_iter(self)
            .map(|m| m.as_str().parse().unwrap())
    }

    pub fn vecs(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.numbers().tuples().map(|(x, y)| Vec2(x, y))
    }

    pub fn paragraphs(&'a self) -> impl Iterator<Item = Self> + 'a {
        self.split("\n\n").map(Self)
    }

    pub fn split_once(&self, delimiter: &'static str) -> (Input<'_>, Input<'_>) {
        let (l, r) = self.0.split_once(delimiter).unwrap();

        (l.into(), r.into())
    }

    pub fn grid(&self) -> Grid<char> {
        Grid::parse(self.0)
    }
}

impl Deref for Input<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

pub fn arrow_to_direction(char: char) -> Option<Vec2> {
    match char {
        '^' => Some(Vec2(-1, 0)),
        '>' => Some(Vec2(0, 1)),
        'v' => Some(Vec2(1, 0)),
        '<' => Some(Vec2(0, -1)),
        _ => None,
    }
}
