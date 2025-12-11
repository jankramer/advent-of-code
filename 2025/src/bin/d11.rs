use std::collections::HashMap;

fn run(input: &str) -> (usize, usize) {
    let mut graph: HashMap<&str, Vec<&str>> = Default::default();

    for line in input.lines() {
        let (l, r) = line.split_once(": ").unwrap();
        graph.insert(l, r.split_whitespace().collect::<Vec<_>>());
    }

    let mut cache = Default::default();

    let (p1, _) = paths(&mut cache, &graph, ("you", false, false));
    let (_, p2) = paths(&mut cache, &graph, ("svr", false, false));

    (p1, p2)
}

fn paths<'a>(
    cache: &mut HashMap<(&'a str, bool, bool), (usize, usize)>,
    graph: &HashMap<&str, Vec<&'a str>>,
    from: (&'a str, bool, bool),
) -> (usize, usize) {
    if let Some(&x) = cache.get(&from) {
        return x;
    }

    let mut total = (0, 0);
    for &nb in graph.get(from.0).unwrap_or(&vec![]) {
        if nb == "out" {
            total.0 += 1;
            total.1 += (from.1 && from.2) as usize;
        } else {
            let subtotal = paths(
                cache,
                graph,
                (nb, from.1 || from.0 == "dac", from.2 || from.0 == "fft"),
            );

            total.0 += subtotal.0;
            total.1 += subtotal.1;
        }
    }

    cache.insert(from, total);

    total
}

fn main() {
    assert_eq!(run(EX1), (5, 0));
    assert_eq!(run(EX2), (0, 2));

    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX1: &str = include_str!("../../inputs/d11.ex1");
const EX2: &str = include_str!("../../inputs/d11.ex2");
const IN: &str = include_str!("../../inputs/d11.in");
