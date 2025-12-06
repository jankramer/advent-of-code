use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::hint::black_box;

const IN: &[u8] = include_bytes!("day10.txt");

fn run(input: &[u8]) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    let grid: Vec<u8> = input
        .trim_ascii()
        .iter()
        .filter(|&b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();
    let width = input.iter().position(|&x| x == b'\n').unwrap();
    let height = grid.len() / width;

    let mut q = vec![];
    let mut peaks = FxHashSet::default();

    for start in grid.iter().positions(|&x| x == 0) {
        q.push((start, 0));

        while let Some((p, h)) = q.pop() {
            let (r, c) = (p / width, p % width);
            let nb4 = [
                (r > 0).then(|| (r - 1) * width + c),
                (r < height - 1).then_some((r + 1) * width + c),
                (c > 0).then(|| p - 1),
                (c < width - 1).then_some(p + 1),
            ]
            .into_iter()
            .flatten();

            for next in nb4 {
                let next_h = grid[next];

                if next_h != h + 1 {
                    continue;
                }

                if next_h == 9 {
                    peaks.insert(next).then(|| p1 += 1);
                    p2 += 1;
                } else {
                    q.push((next, next_h));
                }
            }
        }

        q.clear();
        peaks.clear();
    }

    (p1, p2)
}

fn main() {
    let (p1, p2) = run(IN);
    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    let now = std::time::Instant::now();
    for _ in 0..10000 {
        black_box(run(IN));
    }
    println!("{}ns\n", now.elapsed().as_nanos() / 10000);
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &[u8] = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test() {
        assert_eq!(run(T1), (36, 81));
        assert_eq!(run(IN), (430, 928));
    }
}
