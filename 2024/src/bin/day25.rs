use aoc24::input::Input;
use itertools::Itertools;

const IN: Input = Input::new(include_str!("day25.txt"));

fn run(input: Input) -> (usize, usize) {
    let mut locks = vec![];
    let mut keys = vec![];

    for grid in input.split("\n\n") {
        let lines = grid.lines().collect_vec();
        let is_key = lines.first().unwrap().chars().all(|c| c == '.');

        let mut heights = vec![];
        for col in 0..lines[0].len() {
            let mut height = -1;
            for &row in lines.iter() {
                if row.chars().nth(col).unwrap() == '#' {
                    height += 1;
                }
            }
            heights.push(height);
        }

        (if is_key { &mut keys } else { &mut locks }).push(heights);
    }

    let p1 = locks
        .into_iter()
        .cartesian_product(keys.into_iter())
        .filter(|(lock, key)| (0..lock.len()).all(|n| lock[n] + key[n] <= 5))
        .count();

    (p1, 0)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 25\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 3);
    }
}
