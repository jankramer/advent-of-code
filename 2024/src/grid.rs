use crate::input::Input;
use crate::vector::Vec2;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Grid<T = char> {
    pub data: FxHashMap<Vec2, T>,
    pub width: i64,
    pub height: i64,
}

impl<T> Grid<T> {
    pub fn new(width: i64, height: i64) -> Self {
        Self {
            width,
            height,
            data: Default::default(),
        }
    }

    pub fn fill(self, value: T) -> Self
    where
        T: Copy,
    {
        let (width, height) = (self.width, self.height);

        self.insert(
            (0..height)
                .cartesian_product(0..width)
                .map(|(r, c)| (Vec2(r, c), value)),
        )
    }

    pub fn insert(mut self, data: impl Iterator<Item = (Vec2, T)>) -> Self {
        self.data = self.data.into_iter().chain(data).collect();
        self
    }

    pub fn nb4<'a>(&'a self, pos: &'a Vec2) -> impl Iterator<Item = (&'a Vec2, &'a T)> {
        pos.nb4().filter_map(|p| self.data.get_key_value(&p))
    }
}

impl<T: Eq> Grid<T> {
    pub fn position(&self, needle: &T) -> Option<Vec2> {
        self.data
            .iter()
            .find(|&(_, val)| val == needle)
            .map(|(pos, _)| *pos)
    }
}

impl Grid<char> {
    pub fn parse(input: &str) -> Grid<char> {
        let width = input.find('\n').unwrap() as i64;
        let height = (input.len() as i64 / (width + 1)) + 1;
        let data = input
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(c, chr)| (Vec2(r as i64, c as i64), chr))
            })
            .collect();

        Grid {
            data,
            width,
            height,
        }
    }

    pub fn display(&self) {
        println!("{self}");
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height)
                .map(|r| (0..self.width)
                    .map(|c| self[Vec2(r, c)])
                    .collect::<String>())
                .join("\n")
        )
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height)
                .map(|r| (0..self.width)
                    .map(|c| if self[Vec2(r, c)] { '.' } else { '#' })
                    .collect::<String>())
                .join("\n")
        )
    }
}

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2) -> &Self::Output {
        self.data.get(&index).unwrap()
    }
}

impl<T: Default> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        self.data.entry(index).or_default()
    }
}

impl From<Input<'_>> for Grid<char> {
    fn from(value: Input) -> Self {
        value.grid()
    }
}

impl From<String> for Grid<char> {
    fn from(value: String) -> Self {
        Grid::parse(&value)
    }
}
