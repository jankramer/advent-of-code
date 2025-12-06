use aoc24::input::Input;
use aoc24::vector::Vec2;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const IN: Input = Input::new(include_str!("day20.txt"));

fn run(input: Input) -> (usize, usize) {
    let (mut p1, mut p2) = (FxHashSet::default(), FxHashSet::default());

    let grid = input.grid();
    let start = grid.position(&'S').unwrap();
    let end = grid.position(&'E').unwrap();

    let mut path = vec![start];
    let mut seen = FxHashSet::from_iter([start]);
    while path[path.len() - 1] != end {
        let current = path[path.len() - 1];
        let (next, _) = grid
            .nb4(&current)
            .find(|&(pos, chr)| chr != &'#' && seen.insert(*pos))
            .unwrap();

        path.push(*next);
    }

    let path = FxHashMap::from_iter(path.iter().copied().enumerate().map(|(v, k)| (k, v as i64)));

    for (&start_pos, &start_index) in path.iter() {
        for (dr, dc) in (-20..=20).cartesian_product(-20..=20) {
            let end_pos = start_pos + Vec2(dr, dc);

            let new_dist = end_pos.taxicab(&start_pos);
            if new_dist > 20 {
                continue;
            }

            let Some(&end_index) = path.get(&end_pos) else {
                continue;
            };

            let prev_dist = end_index - start_index;
            if prev_dist < new_dist || prev_dist - new_dist < 100 {
                continue;
            }

            if new_dist <= 2 {
                p1.insert((start_pos, end_pos));
            }

            p2.insert((start_pos, end_pos));
        }
    }

    (p1.len(), p2.len())
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 20\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 1307);
        assert_eq!(real_input_result.1, 986545);
    }
}
