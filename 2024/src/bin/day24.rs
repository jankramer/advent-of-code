use aoc24::input::Input;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const IN: Input = Input::new(include_str!("day24.txt"));
fn binnum(values: &BTreeMap<&str, usize>, char: char) -> usize {
    values
        .iter()
        .filter(|(k, v)| k.starts_with(char))
        .map(|(_, v)| v)
        .enumerate()
        .map(|(i, v)| v * 1 << i)
        .fold(0, |a, b| a | b)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Instr {
    left: usize,
    right: usize,
    op: Op,
}

impl Instr {
    pub fn new(op: Op, left: usize, right: usize) -> Instr {
        Instr {
            left: left.min(right),
            right: left.max(right),
            op,
        }
    }

    pub fn or(left: usize, right: usize) -> Instr {
        Instr::new(Op::OR, left, right)
    }

    pub fn and(left: usize, right: usize) -> Instr {
        Instr::new(Op::AND, left, right)
    }

    pub fn xor(left: usize, right: usize) -> Instr {
        Instr::new(Op::XOR, left, right)
    }

    pub fn has_input(&self, input: usize) -> bool {
        self.left == input || self.right == input
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Op {
    XOR,
    AND,
    OR,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Position {
    Xor,
    And,
    Sum,
    Tmp,
    Carry,
}

fn run(input: Input) -> (usize, String) {
    let (l, r) = input.split_once("\n\n");

    // A   XOR B => 1
    // A   AND B => 2
    // Cin XOR 1 => S
    // Cin AND 1 => 3
    // 2   OR  3 => Cout

    let mut values: BTreeMap<&str, usize> = l
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(l, r)| (l, r.parse::<usize>().unwrap()))
        .collect();

    let opmap = FxHashMap::<&str, Vec<(&str, &str, &str)>>::default();

    for line in r.lines() {}

    let key_regex = Regex::new("[a-z0-9]{3}").unwrap();
    let key_names: Vec<&str> = key_regex
        .find_iter(&input)
        .map(|m| m.as_str())
        .unique()
        .sorted()
        .collect();
    let keys: BTreeMap<&str, usize> = key_names.iter().enumerate().map(|(k, &v)| (v, k)).collect();

    let mut instr: FxHashMap<Instr, usize> = r
        .lines()
        .map(|line| {
            let (inputs, result) = line.split_once(" -> ").unwrap();

            let mut op = Op::AND;

            let (l, r) = inputs
                .split_once(" AND ")
                .or_else(|| {
                    op = Op::XOR;
                    inputs.split_once(" XOR ")
                })
                .or_else(|| {
                    op = Op::OR;
                    inputs.split_once(" OR ")
                })
                .unwrap();

            (Instr::new(op, keys[l], keys[r]), keys[result])
        })
        .collect();

    assert_eq!(instr[&Instr::xor(keys["x00"], keys["y00"])], keys["z00"]);
    let mut carry = instr[&Instr::and(keys["x00"], keys["y00"])];

    let mut expected: FxHashMap<(usize, Position), FxHashMap<&str, usize>> = Default::default();
    for i in 1..45 {
        let x = keys[format!("x{i:02}").as_str()];
        let y = keys[format!("y{i:02}").as_str()];
        let z = keys[format!("z{i:02}").as_str()];

        let xor = instr[&Instr::xor(x, y)];
        let and = instr[&Instr::and(x, y)];
        *expected
            .entry((i, Position::Xor))
            .or_default()
            .entry(key_names[xor])
            .or_default() += 1;

        *expected
            .entry((i, Position::And))
            .or_default()
            .entry(key_names[and])
            .or_default() += 1;

        let sum_instr = instr
            .iter()
            .find_map(|(k, v)| (v == &z).then(|| *k))
            .unwrap();
    }

    let mut n = 1;
    loop {
        println!("z{n:02}");
        let x = keys[format!("x{n:02}").as_str()];
        let y = keys[format!("y{n:02}").as_str()];
        let z = keys[format!("z{n:02}").as_str()];

        let xor = instr[&Instr::xor(x, y)];

        let and = instr[&Instr::and(x, y)];
        let sum_instr = instr
            .iter()
            .find_map(|(k, v)| (v == &z).then(|| *k))
            .unwrap();

        if sum_instr.op != Op::XOR {
            let candidates = instr
                .iter()
                .filter(|(k, v)| k.op == Op::XOR && (k.left == xor || k.right == xor))
                .collect_vec();

            assert_eq!(candidates.len(), 1);
            let (&candidate_instr, &candidate_result) = candidates[0];

            instr.insert(candidate_instr, z);
            instr.insert(sum_instr, candidate_result);
        }

        let sum = instr[&Instr::xor(xor, carry)];
        assert_eq!(sum_instr.op, Op::XOR);
        assert!(sum_instr.left == xor || sum_instr.right == xor);

        assert_eq!(sum_instr, Instr::xor(xor, carry));

        let tmp = instr[&Instr::and(xor, carry)];
        // let sum = instr[&Instr::xor(xor, carry)];

        carry = instr[&Instr::or(and, tmp)];

        n += 1;
        if n > 45 {
            break;
        }
    }
    (0, "".to_string())
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 24\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

fn add(x: usize, y: usize) -> usize {
    if y == 0 {
        return x;
    }

    add(x ^ y, (x & y) << 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 4);
        // assert_eq!(test_input_result.1, 0);

        // let real_input_result = run(IN);
        // assert_eq!(real_input_result.0, 0);
        // assert_eq!(real_input_result.1, 0);
    }
}
