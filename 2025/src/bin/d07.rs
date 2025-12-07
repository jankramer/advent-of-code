use std::collections::{HashMap, HashSet};

fn run(input: &[u8]) -> (usize, usize) {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    let start = input.iter().position(|&c| c == b'S').unwrap();

    let mut graph: HashMap<usize, HashSet<usize>> = Default::default();
    let mut queue = vec![start];
    let mut visited = HashSet::new();

    while let Some(node) = queue.pop() {
        let entry = graph.entry(node).or_default();

        for next in (1..(height - node / width)).map(|dy| node + width * dy) {
            if input.get(next).map(|&b| b == b'^').unwrap_or(false) {
                let (l, r) = (next - 1, next + 1);

                entry.insert(l).then(|| queue.push(l));
                entry.insert(r).then(|| queue.push(r));
                visited.insert(next);

                break;
            }
        }
    }

    (visited.len(), paths(&mut Default::default(), &graph, start))
}

fn paths(
    cache: &mut HashMap<usize, usize>,
    graph: &HashMap<usize, HashSet<usize>>,
    start: usize,
) -> usize {
    if cache.contains_key(&start) {
        return *cache.get(&start).unwrap();
    }

    let nbs = graph.get(&start).unwrap();
    if nbs.is_empty() {
        return 1;
    }

    let result = nbs.iter().map(|nb| paths(cache, graph, *nb)).sum();
    cache.insert(start, result);

    result
}

fn main() {
    assert_eq!(run(EX), (21, 40));
    assert_eq!(run(IN), (1535, 4404709551015));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &[u8] = include_bytes!("../../inputs/d07.ex");
const IN: &[u8] = include_bytes!("../../inputs/d07.in");
