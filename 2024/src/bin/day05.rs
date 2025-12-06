use std::cmp::Ordering;
use std::collections::HashSet;

const IN: &str = include_str!("day05.txt");

pub fn run(input: &str) -> (usize, usize) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(usize, usize)> = rules
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let updates: Vec<Update> = updates
        .lines()
        .map(|l| l.split(",").map(|c| c.parse().unwrap()).collect::<Vec<_>>())
        .map(|numbers| Update {
            expected: sort(&numbers, &rules),
            actual: numbers,
        })
        .collect();

    (
        updates
            .iter()
            .filter(|x| x.is_valid())
            .map(|x| x.actual[x.actual.len() / 2])
            .sum(),
        updates
            .iter()
            .filter(|x| !x.is_valid())
            .map(|x| x.expected[x.expected.len() / 2])
            .sum(),
    )
}

fn sort(numbers: &[usize], rules: &HashSet<(usize, usize)>) -> Vec<usize> {
    let mut numbers: Vec<_> = numbers.to_vec();

    numbers.sort_by(|a, b| match rules.contains(&(*a, *b)) {
        true => Ordering::Less,
        false => Ordering::Greater,
    });

    numbers
}

pub struct Update {
    actual: Vec<usize>,
    expected: Vec<usize>,
}

impl Update {
    pub fn is_valid(&self) -> bool {
        self.actual == self.expected
    }
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 06\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (143, 123));
        assert_eq!(run(IN), (5948, 3062));
    }
}
