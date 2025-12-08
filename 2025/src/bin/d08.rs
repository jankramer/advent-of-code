use std::collections::{HashMap, HashSet};

fn run(input: &str, n: usize) -> (usize, usize) {
    let coords: Vec<[usize; 3]> = input
        .lines()
        .map(|l| {
            let (x, rest) = l.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();

            [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]
        })
        .collect();

    let mut distances: Vec<_> = coords
        .iter()
        .enumerate()
        .flat_map(|(idx_a, a)| coords.iter().skip(idx_a + 1).map(move |b| (a, b)))
        .map(|(a, b)| (distance(a, b) as usize, a, b))
        .collect();

    distances.sort_unstable();

    let mut cliques: Vec<HashSet<&[usize; 3]>> =
        coords.iter().map(|c| HashSet::from_iter([c])).collect();

    let mut mapping: HashMap<&[usize; 3], usize> =
        (0..cliques.len()).map(|c| (&coords[c], c)).collect();

    let mut empty_cliques = HashSet::<usize>::default();

    let mut p1 = 0;
    for (i, (_, coords_a, coords_b)) in distances.into_iter().enumerate() {
        if i == n {
            let mut lengths = cliques.iter().map(|c| c.len()).collect::<Vec<_>>();
            lengths.sort_unstable();
            p1 = lengths.iter().rev().take(3).product();
        }

        if mapping[coords_a] == mapping[coords_b] {
            continue;
        }

        empty_cliques.insert(mapping[coords_b]);

        let [a, b] = cliques
            .get_disjoint_mut([mapping[coords_a], mapping[coords_b]])
            .unwrap();

        for foo in b.drain() {
            a.insert(foo);
            mapping.insert(foo, mapping[coords_a]);
        }

        if empty_cliques.len() == cliques.len() - 1 {
            return (p1, coords_a[0] * coords_b[0]);
        }
    }

    unreachable!()
}

fn distance<const N: usize>(a: &[usize; N], b: &[usize; N]) -> f64 {
    ((0..N).map(|i| a[i].abs_diff(b[i]).pow(2)).sum::<usize>() as f64).sqrt()
}

fn main() {
    assert_eq!(run(EX, 10), (40, 25272));
    assert_eq!(run(IN, 1000), (50760, 3206508875));

    aoc25::print(run(IN, 1000));
    aoc25::bench(|| std::hint::black_box(run(IN, 1000)));
}

const EX: &str = include_str!("../../inputs/d08.ex");
const IN: &str = include_str!("../../inputs/d08.in");
