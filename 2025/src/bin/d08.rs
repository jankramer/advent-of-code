use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

fn run(input: &str, n: usize) -> (usize, usize) {
    let coords: Vec<[usize; 3]> = input
        .lines()
        .map(|l| {
            let (x, rest) = l.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();

            [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]
        })
        .collect();

    let mut distances: BinaryHeap<_> = coords
        .iter()
        .enumerate()
        .flat_map(|(idx_a, a)| {
            coords
                .iter()
                .enumerate()
                .skip(idx_a + 1)
                .map(move |(idx_b, b)| (a, b, idx_a, idx_b))
        })
        .map(|(a, b, idx_a, idx_b)| Reverse((distance(a, b), idx_a, idx_b)))
        .collect();

    let mut clusters: Vec<HashSet<usize, BuildHasherDefault<DumbHash>>> =
        (0..coords.len()).map(|i| HashSet::from_iter([i])).collect();

    let mut mapping: HashMap<usize, usize, BuildHasherDefault<DumbHash>> =
        (0..clusters.len()).map(|c| (c, c)).collect();

    let mut empty = HashSet::<usize, BuildHasherDefault<DumbHash>>::default();

    let mut p1 = 0;

    let mut i = 0;
    while let Some(Reverse((_, idx_a, idx_b))) = distances.pop() {
        if i == n {
            let mut lengths = clusters.iter().map(|c| c.len()).collect::<Vec<_>>();
            lengths.sort_unstable();
            p1 = lengths.iter().rev().take(3).product();
        }

        if mapping[&idx_a] == mapping[&idx_b] {
            i += 1;
            continue;
        }

        empty.insert(mapping[&idx_b]);

        let [a, b] = clusters
            .get_disjoint_mut([mapping[&idx_a], mapping[&idx_b]])
            .unwrap();

        for foo in b.drain() {
            a.insert(foo);
            mapping.insert(foo, mapping[&idx_a]);
        }

        if empty.len() == clusters.len() - 1 {
            return (p1, coords[idx_a][0] * coords[idx_b][0]);
        }

        i += 1;
    }

    unreachable!()
}

fn distance<const N: usize>(a: &[usize; N], b: &[usize; N]) -> usize {
    (0..N).map(|i| a[i].abs_diff(b[i]).pow(2)).sum::<usize>()
}

#[derive(Default)]
struct DumbHash(usize);

impl Hasher for DumbHash {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _: &[u8]) {
        panic!()
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i
    }
}

fn main() {
    assert_eq!(run(EX, 10), (40, 25272));
    assert_eq!(run(IN, 1000), (50760, 3206508875));

    aoc25::print(run(IN, 1000));
    aoc25::bench(|| std::hint::black_box(run(IN, 1000)));
}

const EX: &str = include_str!("../../inputs/d08.ex");
const IN: &str = include_str!("../../inputs/d08.in");
