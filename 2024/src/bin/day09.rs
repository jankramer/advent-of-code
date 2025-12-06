use itertools::{repeat_n, Itertools};
use std::collections::VecDeque;

const IN: &str = include_str!("day09.txt");

#[derive(Clone, Debug)]
struct Entry {
    idx: usize,
    files: usize,
    free: usize,
}

fn run(input: &str) -> (usize, usize) {
    let digits = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x as usize))
        .collect_vec();

    let entries: Vec<_> = digits
        .chunks(2)
        .enumerate()
        .map(|(idx, x)| Entry {
            idx,
            files: x[0],
            free: x.get(1).copied().unwrap_or_default(),
        })
        .collect();

    (
        checksum(solve_a(&entries)),
        checksum(entries_to_blocks(&solve_b(&entries))),
    )
}

fn solve_a(entries: &[Entry]) -> Vec<Option<usize>> {
    let mut blocks = entries_to_blocks(entries);

    let max_idx = blocks.iter().rposition(|x| !x.is_none()).unwrap();

    let free_blocks = blocks[..max_idx]
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| v.is_none().then_some(idx));

    let last_blocks = blocks
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, v)| v.is_some().then_some(i));

    let pairs: Vec<_> = free_blocks
        .zip(last_blocks)
        .take_while(|(a, b)| a < b)
        .collect();

    for (free, to_move) in pairs {
        blocks.swap(free, to_move);
    }

    blocks
}

fn solve_b(entries: &[Entry]) -> Vec<Entry> {
    let mut entries: VecDeque<_> = entries.iter().cloned().collect();
    let mut back = VecDeque::new();

    'outer: while let Some(mut right) = entries.pop_back() {
        for (i, left) in entries.iter_mut().enumerate() {
            if left.free < right.files {
                continue;
            }

            back.push_front(Entry {
                idx: 0,
                files: 0,
                free: right.files + right.free,
            });

            right.free = left.free - right.files;
            left.free = 0;

            entries.insert(i + 1, right);

            continue 'outer;
        }

        back.push_front(right);
    }

    entries.into_iter().chain(back).collect()
}

fn entries_to_blocks(entries: &[Entry]) -> Vec<Option<usize>> {
    entries
        .iter()
        .flat_map(|x| repeat_n(Some(x.idx), x.files).chain(repeat_n(None, x.free)))
        .collect()
}

fn checksum(blocks: Vec<Option<usize>>) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(a, b)| a * b.unwrap_or_default())
        .sum()
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 09\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}ms\n", elapsed.as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = "2333133121414131402";

    #[test]
    fn test() {
        let (test_p1, test_p2) = run(T1);
        assert_eq!(test_p1, 1928);
        assert_eq!(test_p2, 2858);

        let (p1, p2) = run(IN);
        assert_eq!(p1, 6154342787400);
        assert_eq!(p2, 6183632723350);
    }
}
