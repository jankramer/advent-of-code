use std::cmp::Ordering;

use aoc24::input::Input;
use aoc24::vector::{StateVec, Vec2};
use itertools::Itertools;
use rustc_hash::FxHashSet;

const IN: Input = Input::new(include_str!("day14.txt"));
const THRESHOLD: usize = 50;

fn run(input: Input, grid_size: Vec2) -> (usize, usize) {
    let robots: Vec<StateVec> = input
        .numbers()
        .tuples()
        .map(|(x, y, dx, dy)| StateVec::new(Vec2(x, y), Vec2(dx, dy)))
        .collect();

    let p1 = robots
        .iter()
        .map(|r| r.at_time(100) % grid_size)
        .filter_map(|x| quadrant(x, grid_size))
        .counts()
        .values()
        .product();

    let p2 = (0..100_000)
        .find(|&t| {
            let set: FxHashSet<_> = robots
                .iter()
                .map(|r| r.at_time(t as i64) % grid_size)
                .collect();

            set.iter()
                .filter(|p| p.nb4().all(|nb| set.contains(&nb)))
                .take(THRESHOLD)
                .count()
                == THRESHOLD
        })
        .unwrap();

    (p1, p2)
}

fn quadrant(p: Vec2, grid_size: Vec2) -> Option<(Ordering, Ordering)> {
    match (p.0.cmp(&(grid_size.0 / 2)), p.1.cmp(&(grid_size.1 / 2))) {
        (Ordering::Equal, _) | (_, Ordering::Equal) => None,
        (a, b) => Some((a, b)),
    }
}

fn main() {
    let (p1, p2) = run(IN, Vec2(101, 103));

    println!("Day 14\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    let now = std::time::Instant::now();
    for _ in 0..20 {
        run(IN, Vec2(101, 103));
    }
    println!("{}ms\n", now.elapsed().as_millis() / 20);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let real_input_result = run(IN, Vec2(101, 103));
        assert_eq!(real_input_result.0, 221655456);
        assert_eq!(real_input_result.1, 7858);
    }
}
