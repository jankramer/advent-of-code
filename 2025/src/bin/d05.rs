use std::cmp::Ordering;

fn run(input: &str) -> (usize, usize) {
    let (left, right) = input.split_once("\n\n").unwrap();

    let ids = right.lines().map(|l| l.parse::<usize>().unwrap());
    let mut ranges_input: Vec<(usize, usize)> = left
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    ranges_input.sort_unstable();

    let mut ranges = vec![ranges_input[0]];
    for range in ranges_input.into_iter().skip(1) {
        let idx = ranges.len() - 1;
        match range.0 <= ranges[idx].1 {
            true => ranges[idx].1 = range.1.max(ranges[idx].1),
            false => ranges.push(range),
        }
    }

    let in_range = |x: &usize| {
        ranges
            .binary_search_by(|(min, max)| match x >= min && x <= max {
                true => Ordering::Equal,
                false => min.cmp(&x),
            })
            .is_ok()
    };

    (
        ids.filter(in_range).count(),
        ranges.iter().map(|(a, b)| b - a + 1).sum(),
    )
}

fn main() {
    assert_eq!(run(EX), (3, 14));
    assert_eq!((773, 332067203034711), run(IN));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &str = include_str!("../../inputs/d05.ex");
const IN: &str = include_str!("../../inputs/d05.in");
