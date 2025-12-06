use aoc24::input::Input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;

const IN: Input = Input::new(include_str!("day23.txt"));

fn run(input: Input) -> (usize, String) {
    let mut links: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();

        links.entry(l).or_default().insert(r);
        links.entry(r).or_default().insert(l);
    }

    let mut sets3: FxHashSet<BTreeSet<&str>> = FxHashSet::default();
    let mut p2 = "".to_string();
    for (&root, peers) in &links {
        let mut sets = vec![];
        for peer in peers {
            sets.push(BTreeSet::from_iter([root, peer]));
        }

        for peer in peers {
            for set in sets.iter_mut() {
                if set.iter().all(|p| links[peer].contains(p)) {
                    set.insert(peer);
                }
            }
        }

        for set in &sets {
            for set3 in set.iter().copied().combinations(3) {
                if set3.iter().any(|y| y.starts_with('t')) {
                    sets3.insert(set3.into_iter().collect());
                }
            }

            let new_set = set.iter().join(",");
            if new_set.len() > p2.len() {
                p2 = new_set;
            }
        }
    }

    let p1 = sets3.len();

    (p1, p2)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 23\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 7);
        assert_eq!(test_input_result.1, "co,de,ka,ta");

        // let real_input_result = run(IN);
        // assert_eq!(real_input_result.0, 0);
        // assert_eq!(real_input_result.1, 0);
    }
}
