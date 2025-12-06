use aoc24::input::Input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const IN: Input = Input::new(include_str!("day22.txt"));

fn run(input: Input) -> (i64, i64) {
    let mut p1 = 0;

    let mut prizes = FxHashMap::<(i64, i64, i64, i64), i64>::default();
    for num in input.numbers() {
        let mut last_digits = vec![num % 10];
        let mut num = num;

        for _ in 0..2000 {
            num = step(num);
            last_digits.push(num % 10);
        }

        p1 += num;

        let mut seen = FxHashSet::default();

        for (deltas, bananas) in last_digits
            .windows(2)
            .map(|xs| xs[1] - xs[0])
            .tuple_windows::<(i64, i64, i64, i64)>()
            .enumerate()
            .map(|(i, deltas)| (deltas, last_digits[i + 4]))
        {
            if !seen.insert(deltas) {
                continue;
            }

            *prizes.entry(deltas).or_default() += bananas;
        }
    }

    let p2 = *prizes.values().max().unwrap();

    (p1, p2)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 22s\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

fn step(num: i64) -> i64 {
    let num = ((num * 64) ^ num) % 16777216;
    let num = ((num / 32) ^ num) % 16777216;
    ((num * 2048) ^ num) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"1
10
100
2024"#,
    );

    const T2: Input = Input::new(
        r#"1
2
3
2024"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 37327623);

        let test_input_result = run(T2);
        assert_eq!(test_input_result.1, 23);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 18261820068);
        assert_eq!(real_input_result.1, 2044);
    }
}
