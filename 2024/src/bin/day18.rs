use aoc24::grid::Grid;
use aoc24::input::Input;
use aoc24::vector::Vec2;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

const IN: Input = Input::new(include_str!("day18.txt"));

fn run(input: &Input, size: i64, n: usize) -> (usize, String) {
    let shortest_path = |n: usize| {
        let grid = Grid::new(size, size)
            .fill('.')
            .insert(input.vecs().take(n).map(|p| (p, '#')));

        let mut seen = FxHashSet::default();
        let mut q = VecDeque::from([(Vec2(0, 0), 0)]);

        while let Some((current, dist)) = q.pop_front() {
            if current == Vec2(size - 1, size - 1) {
                return Some(dist);
            }

            for nb in current.nb4() {
                if !seen.insert(nb) {
                    continue;
                }

                if !grid.data.get(&nb).is_some_and(|c| c == &'.') {
                    continue;
                }

                q.push_back((nb, dist + 1));
            }
        }

        None
    };

    let partition_point = (n + 1..input.lines().count())
        .collect::<Vec<_>>()
        .partition_point(|&n| shortest_path(n).is_some());

    (
        shortest_path(n).unwrap(),
        input.lines().nth(n + partition_point).unwrap().to_string(),
    )
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(&IN, 71, 1024);

    let elapsed = now.elapsed();

    println!("Day 18\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(&T1, 7, 12);
        assert_eq!(test_input_result.0, 22);
        assert_eq!(test_input_result.1, "6,1");

        let real_input_result = run(&IN, 71, 1024);
        assert_eq!(real_input_result.0, 340);
        assert_eq!(real_input_result.1, "34,32");
    }
}
