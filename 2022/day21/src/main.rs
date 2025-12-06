use crate::Job::MathJob;
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(solve_a(INPUT_TEST), 152);
    println!("Part A: {}", solve_a(INPUT));

    // assert_eq!(solve_b(INPUT_TEST), 301);
    println!("Part B: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> isize {
    let monkeys = parse(input);

    monkeys.get("root").unwrap().result(&monkeys)
}

fn solve_b(input: &str) -> isize {
    let monkeys = parse(input);

    let root_monkey = monkeys.get("root").unwrap();
    match &root_monkey.job {
        Job::Scalar(_) => 0,
        MathJob(MathOp { left, right, op }) => {
            let (left_val, right_val) = part_b_helper(monkeys.clone(), 0, left, right);
            let left_smaller = left_val < right_val;

            let exp = (0..20)
                .map(|i| part_b_helper(monkeys.clone(), 10_isize.pow(i), left, right))
                .take_while(|(left, right)| {
                    if left_smaller {
                        left < right
                    } else {
                        left > right
                    }
                })
                .count();

            let mut lower_bound = 0;
            let mut upper_bound = 10_isize.pow(exp as u32 + 2);

            loop {
                let i = lower_bound + ((upper_bound - lower_bound) / 2);

                let (left_val, right_val) = part_b_helper(monkeys.clone(), i, left, right);

                if left_val == right_val {
                    return i;
                }

                if left_smaller && left_val < right_val {
                    lower_bound = i;
                } else {
                    upper_bound = i;
                }
            }

            loop {}
        }
    };

    0
}

fn part_b_helper(
    mut monkeys: HashMap<String, Monkey>,
    i: isize,
    left: &str,
    right: &str,
) -> (isize, isize) {
    monkeys.insert(
        "humn".to_string(),
        Monkey {
            id: "humn".to_string(),
            job: Job::Scalar(i),
        },
    );

    (
        monkeys.get(left).unwrap().result(&monkeys),
        monkeys.get(right).unwrap().result(&monkeys),
    )
}

fn parse(input: &str) -> HashMap<String, Monkey> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").unwrap();

            if let Ok(number) = right.parse::<isize>() {
                return (
                    left.to_string(),
                    Monkey {
                        id: left.to_string(),
                        job: Job::Scalar(number),
                    },
                );
            }

            let parts = right.split_whitespace().collect_vec();

            (
                left.to_string(),
                Monkey {
                    id: left.to_string(),
                    job: MathJob(MathOp {
                        op: match parts[1] {
                            "+" => Op::Add,
                            "-" => Op::Sub,
                            "*" => Op::Mul,
                            "/" => Op::Div,
                            _ => Op::Add,
                        },
                        left: parts[0].to_string(),
                        right: parts[2].to_string(),
                    }),
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    job: Job,
}

impl Monkey {
    fn result(&self, monkeys: &HashMap<String, Monkey>) -> isize {
        match self.clone().job {
            Job::Scalar(num) => num,
            MathJob(math_op) => match math_op.op {
                Op::Add => {
                    monkeys.get(&math_op.left).unwrap().result(monkeys)
                        + monkeys.get(&math_op.right).unwrap().result(monkeys)
                }

                Op::Sub => {
                    monkeys.get(&math_op.left).unwrap().result(monkeys)
                        - monkeys.get(&math_op.right).unwrap().result(monkeys)
                }
                Op::Mul => {
                    monkeys.get(&math_op.left).unwrap().result(monkeys)
                        * monkeys.get(&math_op.right).unwrap().result(monkeys)
                }
                Op::Div => {
                    monkeys.get(&math_op.left).unwrap().result(monkeys)
                        / monkeys.get(&math_op.right).unwrap().result(monkeys)
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Job {
    Scalar(isize),
    MathJob(MathOp),
}

#[derive(Debug, Clone)]
struct MathOp {
    op: Op,
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Scalar(isize);
