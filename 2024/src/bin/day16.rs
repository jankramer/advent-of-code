use aoc24::input::Input;
use aoc24::vector::{StateVec, Vec2};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const IN: Input = Input::new(include_str!("day16.txt"));

fn run(input: Input) -> (usize, usize) {
    let grid = input.grid();

    let mut prev: FxHashMap<StateVec, FxHashSet<(StateVec, usize)>> = FxHashMap::default();
    let start = StateVec::new(grid.position(&'S').unwrap(), Vec2(0, 1));
    let end = grid.position(&'E').unwrap();

    let mut q = BinaryHeap::from([(Reverse(0), start)]);
    let mut scores = FxHashMap::default();
    scores.insert(q.peek().unwrap().1, 0);

    let mut best_score = None;
    while let Some((_, current)) = q.pop() {
        let current_score = *scores.get(&current).unwrap();
        if current_score > best_score.unwrap_or(usize::MAX) {
            continue;
        }

        if current.r == end {
            best_score = Some(current_score);
        }

        for (nb, score) in [
            (current.rotate_left(), current_score + 1000),
            (current.rotate_right(), current_score + 1000),
            (current.step(1), current_score + 1),
        ] {
            if grid[nb.r] == '#' || score > *scores.get(&nb.step(1)).unwrap_or(&usize::MAX) {
                continue;
            }

            if nb.r == current.r && grid[nb.at_time(1)] == '#' {
                continue;
            }

            q.push((Reverse(score), nb));
            scores.insert(nb, score);
            prev.entry(nb).or_default().insert((current, score));
        }
    }

    let best_score = best_score.unwrap();

    let mut q = vec![];
    for prev_nodes in prev.values() {
        for &(state, score) in prev_nodes.iter() {
            if score == best_score {
                q.push(state);
            }
        }
    }

    let mut visited = FxHashSet::default();
    visited.insert(end);
    while let Some(current) = q.pop() {
        visited.insert(current.r);

        q.extend(
            prev.get(&current)
                .iter()
                .flat_map(|prev| prev.iter().map(|&(x, _)| x)),
        );
    }

    (best_score, visited.len())
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
    );

    const T2: Input = Input::new(
        r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 7036);
        assert_eq!(test_input_result.1, 45);

        let test_input_result = run(T2);
        assert_eq!(test_input_result.0, 11048);
        assert_eq!(test_input_result.1, 64);

        // let real_input_result = run(IN);
        // assert_eq!(real_input_result.0, 0);
        // assert_eq!(real_input_result.1, 0);
    }
}
