use aoc24::input::Input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const IN: Input = Input::new(include_str!("day21.txt"));

type Cache = FxHashMap<(char, char, usize), usize>;
type Neighbors = FxHashMap<char, Vec<(char, char)>>;

fn run(input: Input) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let nbs_num = FxHashMap::from_iter(vec![
        ('A', vec![('<', '0'), ('^', '3')]),
        ('0', vec![('^', '2'), ('>', 'A')]),
        ('1', vec![('>', '2'), ('^', '4')]),
        ('2', vec![('<', '1'), ('^', '5'), ('>', '3'), ('v', '0')]),
        ('3', vec![('<', '2'), ('^', '6'), ('v', 'A')]),
        ('4', vec![('>', '5'), ('^', '7'), ('v', '1')]),
        ('5', vec![('<', '4'), ('^', '8'), ('>', '6'), ('v', '2')]),
        ('6', vec![('<', '5'), ('^', '9'), ('v', '3')]),
        ('7', vec![('>', '8'), ('v', '4')]),
        ('8', vec![('<', '7'), ('v', '5'), ('>', '9')]),
        ('9', vec![('<', '8'), ('v', '6')]),
    ]);

    let nbs_dir = FxHashMap::from_iter(vec![
        ('^', vec![('>', 'A'), ('v', 'v')]),
        ('A', vec![('<', '^'), ('v', '>')]),
        ('<', vec![('>', 'v')]),
        ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ('>', vec![('<', 'v'), ('^', 'A')]),
    ]);

    for line in input.lines() {
        let code = line.trim_end_matches('A').parse::<usize>().unwrap();
        p1 += code * count(&nbs_num, &nbs_dir, line, 2);
        p2 += code * count(&nbs_num, &nbs_dir, line, 25);
    }

    (p1, p2)
}

fn count(num_graph: &Neighbors, dir_graph: &Neighbors, input: &str, depth: usize) -> usize {
    let mut cache = FxHashMap::default();
    let mut sum = 0;
    for (a, b) in format!("A{input}").chars().tuple_windows() {
        sum += count_inner(&mut cache, num_graph, dir_graph, depth, a, b, 0);
    }

    sum
}

fn count_inner(
    cache: &mut Cache,
    nbs_num: &Neighbors,
    nbs_dir: &Neighbors,
    max_depth: usize,
    from: char,
    to: char,
    depth: usize,
) -> usize {
    if cache.contains_key(&(from, to, depth)) {
        return cache[&(from, to, depth)];
    }

    let nbs = if depth == 0 { nbs_num } else { nbs_dir };

    if depth == max_depth {
        return all_shortest_paths(nbs, from, to)
            .iter()
            .map(|p| p.len())
            .min()
            .unwrap();
    }

    let mut min_length = usize::MAX;
    for path in all_shortest_paths(nbs, from, to) {
        let current = [&'A']
            .into_iter()
            .chain(path.iter())
            .tuple_windows()
            .map(|(&a, &b)| count_inner(cache, nbs_num, nbs_dir, max_depth, a, b, depth + 1))
            .sum();

        if current < min_length {
            min_length = current;
        }
    }

    cache.insert((from, to, depth), min_length);

    min_length
}

fn all_shortest_paths(
    nbs: &FxHashMap<char, Vec<(char, char)>>,
    from: char,
    to: char,
) -> Vec<Vec<char>> {
    if from == to {
        return vec![vec!['A']];
    }

    let mut queue = VecDeque::from([(from, vec![])]);
    let mut paths = vec![];
    let mut min_length = usize::MAX;

    while let Some((current, path)) = queue.pop_front() {
        if path.len() + 2 > min_length {
            continue;
        }

        for &(key, val) in &nbs[&current] {
            let mut path = path.clone();

            if val == to {
                path.extend([key, 'A']);
                min_length = path.len();
                paths.push(path);
                continue;
            }

            path.push(key);
            queue.push_back((val, path));
        }
    }

    paths
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 21\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"029A
980A
179A
456A
379A"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 126384);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 136780);
        assert_eq!(real_input_result.1, 167538833832712);
    }
}
