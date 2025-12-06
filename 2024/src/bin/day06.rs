use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BTreeSet;

const IN: &str = include_str!("day06.txt");

fn run(input: &str) -> (usize, usize) {
    let mut start = (0, 0);
    let mut rows = vec![];
    let mut cols = vec![];
    for (r, l) in input.lines().enumerate() {
        rows.push(BTreeSet::new());
        for (c, v) in l.chars().enumerate() {
            if r == 0 {
                cols.push(BTreeSet::new());
            }

            if v == '#' {
                rows[r].insert(c);
                cols[c].insert(r);
            }

            if v == '^' {
                start = (r, c);
            }
        }
    }

    let Result::Exit(path) = walk_grid(&cols, &rows, start, Heading::N) else {
        panic!("part 1 should exit");
    };

    let full_path: Vec<_> = path
        .windows(2)
        .flat_map(|pts| points_between(pts[0].0, pts[1].0).map(|p| (p, pts[0].1)))
        .collect();

    let p1 = full_path
        .iter()
        .map(|x| x.0)
        .collect::<FxHashSet<_>>()
        .len();

    let mut p2: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut visited = FxHashSet::default();
    for (p, dir) in full_path {
        if !visited.insert(p) {
            continue;
        }

        rows[p.0].insert(p.1);
        cols[p.1].insert(p.0);

        let start = match dir {
            Heading::N => (p.0 + 1, p.1),
            Heading::E => (p.0, p.1 - 1),
            Heading::S => (p.0 - 1, p.1),
            Heading::W => (p.0, p.1 + 1),
        };

        let result = walk_grid(&cols, &rows, start, dir.rotate());
        if matches!(result, Result::Loop) {
            p2.insert(p);
        }

        rows[p.0].remove(&p.1);
        cols[p.1].remove(&p.0);
    }

    (p1, p2.len())
}

fn walk_grid(
    cols: &[BTreeSet<usize>],
    rows: &[BTreeSet<usize>],
    start: (usize, usize),
    dir: Heading,
) -> Result {
    let mut path = vec![(start, dir)];
    let mut visited = FxHashSet::default();
    let mut pos = start;
    let mut dir = dir;
    let height = rows.len();
    let width = cols.len();
    loop {
        let next = match dir {
            Heading::N => cols[pos.1].iter().rev().find(|&&r| r < pos.0),
            Heading::S => cols[pos.1].iter().find(|&&r| r > pos.0),
            Heading::E => rows[pos.0].iter().find(|&&c| c > pos.1),
            Heading::W => rows[pos.0].iter().rev().find(|&&c| c < pos.1),
        }
        .copied();

        let Some(next) = next else {
            path.push((
                match dir {
                    Heading::N => (0, pos.1),
                    Heading::S => (height - 1, pos.1),
                    Heading::E => (pos.0, width - 1),
                    Heading::W => (pos.0, 0),
                },
                dir,
            ));

            break;
        };

        let next_pos = match dir {
            Heading::N => (next + 1, pos.1),
            Heading::S => (next - 1, pos.1),
            Heading::E => (pos.0, next - 1),
            Heading::W => (pos.0, next + 1),
        };

        pos = next_pos;
        dir = dir.rotate();

        path.push((next_pos, dir));

        if !visited.insert((next_pos, dir)) {
            return Result::Loop;
        }
    }

    Result::Exit(path)
}

#[derive(Debug)]
enum Result {
    Loop,
    Exit(Vec<((usize, usize), Heading)>),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    pub fn rotate(&self) -> Self {
        match self {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N,
        }
    }
}

fn points_between(p1: (usize, usize), p2: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let dr = p1.0.cmp(&p2.0);
    let dc = p1.1.cmp(&p2.1);

    let mut done = false;
    let (mut r, mut c) = p1;

    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let result = Some((r, c));

        if r == p2.0 && c == p2.1 {
            done = true;
            return result;
        }

        match (dr, dc) {
            (Ordering::Greater, _) => {
                r -= 1;
            }
            (Ordering::Less, _) => {
                r += 1;
            }
            (_, Ordering::Greater) => {
                c -= 1;
            }
            (_, Ordering::Less) => {
                c += 1;
            }
            _ => unreachable!(),
        };

        result
    })
}

fn main() {
    let (p1, p2) = run(IN);

    assert_eq!(5453, p1);
    assert_eq!(2188, p2);

    println!("Day 06\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    let now = std::time::Instant::now();
    for _ in 0..10 {
        run(IN);
    }
    let elapsed = now.elapsed();

    println!("{}µs\n", elapsed.as_micros() / 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (41, 6));
        assert_eq!(run(IN), (5453, 2188));
    }
}
