use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;
use std::ops::{Add, Sub};

const IN: &str = include_str!("day12.txt");

fn run(input: &str) -> (usize, usize) {
    let grid: FxHashMap<Point, char> = input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(move |(c, v)| (Point::new(r as i64, c as i64), v))
        })
        .collect();

    let mut unvisited: BTreeSet<Point> = grid.keys().copied().collect();
    let mut regions = vec![];

    while let Some(initial_plot) = unvisited.pop_first() {
        let current_type = grid.get(&initial_plot).unwrap();
        let mut region = Region {
            kind: *current_type,
            area: 1,
            ..Default::default()
        };
        let mut visited_nbs = FxHashSet::default();

        let mut q = vec![initial_plot];
        while let Some(current) = q.pop() {
            for nb in nb4(current) {
                let Some(nb_type) = grid.get(&nb) else {
                    region.perimeter += 1;
                    region.edges.insert(Edge {
                        point: current,
                        side: current.edge(nb).unwrap(),
                    });
                    continue;
                };

                if nb_type != current_type {
                    region.perimeter += 1;
                    region.edges.insert(Edge {
                        point: current,
                        side: current.edge(nb).unwrap(),
                    });
                    continue;
                }

                if !unvisited.remove(&nb) {
                    continue;
                };

                region.area += 1;

                if !visited_nbs.insert(nb) {
                    continue;
                }

                q.push(nb);
            }
        }

        let mut sides = 0;

        while let Some(mut current) = region.edges.pop_first() {
            sides += 1;

            while let Some(next) = current
                .possible_next()
                .into_iter()
                .filter_map(|e| region.edges.take(&e))
                .next()
            {
                if next.side != current.side {
                    sides += 1;
                }

                current = next;
            }
        }

        region.sides = sides;

        regions.push(region);
    }

    (
        regions.iter().map(|r| r.area * r.perimeter).sum(),
        regions.iter().map(|r| r.area * r.sides).sum(),
    )
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Region {
    kind: char,
    area: usize,
    perimeter: usize,
    edges: BTreeSet<Edge>,
    sides: usize,
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    r: i64,
    c: i64,
}

impl Point {
    pub fn new(r: i64, c: i64) -> Self {
        Point { r, c }
    }

    pub fn edge(&self, other: Point) -> Option<Direction> {
        match (other.r - self.r, other.c - self.c) {
            (1, 0) => Some(Direction::S),
            (-1, 0) => Some(Direction::N),
            (0, -1) => Some(Direction::W),
            (0, 1) => Some(Direction::E),
            _ => None,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.r + rhs.r, self.c + rhs.c)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.r - rhs.r, self.c - rhs.c)
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Edge {
    point: Point,
    side: Direction,
}

impl Edge {
    pub fn possible_next(&self) -> Vec<Edge> {
        match self.side {
            Direction::N => vec![
                Edge {
                    point: self.point + Point::new(0, 1),
                    side: self.side,
                },
                Edge {
                    point: self.point + Point::new(0, -1),
                    side: self.side,
                },
                Edge {
                    point: self.point,
                    side: Direction::W,
                },
                Edge {
                    point: self.point,
                    side: Direction::E,
                },
                Edge {
                    point: self.point + Point::new(-1, -1),
                    side: Direction::E,
                },
                Edge {
                    point: self.point + Point::new(-1, 1),
                    side: Direction::W,
                },
            ],
            Direction::E => vec![
                Edge {
                    point: self.point + Point::new(1, 0),
                    side: self.side,
                },
                Edge {
                    point: self.point + Point::new(-1, 0),
                    side: self.side,
                },
                Edge {
                    point: self.point,
                    side: Direction::N,
                },
                Edge {
                    point: self.point,
                    side: Direction::S,
                },
                Edge {
                    point: self.point + Point::new(1, 1),
                    side: Direction::N,
                },
                Edge {
                    point: self.point + Point::new(-1, 1),
                    side: Direction::S,
                },
            ],
            Direction::S => vec![
                Edge {
                    point: self.point + Point::new(0, 1),
                    side: self.side,
                },
                Edge {
                    point: self.point + Point::new(0, -1),
                    side: self.side,
                },
                Edge {
                    point: self.point,
                    side: Direction::W,
                },
                Edge {
                    point: self.point,
                    side: Direction::E,
                },
                Edge {
                    point: self.point + Point::new(1, -1),
                    side: Direction::E,
                },
                Edge {
                    point: self.point + Point::new(1, 1),
                    side: Direction::W,
                },
            ],
            Direction::W => vec![
                Edge {
                    point: self.point + Point::new(1, 0),
                    side: self.side,
                },
                Edge {
                    point: self.point + Point::new(-1, 0),
                    side: self.side,
                },
                Edge {
                    point: self.point,
                    side: Direction::N,
                },
                Edge {
                    point: self.point,
                    side: Direction::S,
                },
                Edge {
                    point: self.point + Point::new(-1, -1),
                    side: Direction::S,
                },
                Edge {
                    point: self.point + Point::new(1, -1),
                    side: Direction::N,
                },
            ],
        }
    }
}

fn nb4(p: Point) -> Vec<Point> {
    vec![
        Point::new(p.r - 1, p.c),
        Point::new(p.r + 1, p.c),
        Point::new(p.r, p.c - 1),
        Point::new(p.r, p.c + 1),
    ]
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 12\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const T2: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    const T3: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 1930);
        assert_eq!(test_input_result.1, 1206);

        let test_input_result = run(T2);
        assert_eq!(test_input_result.1, 236);

        let test_input_result = run(T3);
        assert_eq!(test_input_result.1, 368);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 1452678);
        assert_eq!(real_input_result.1, 873584);
    }
}
