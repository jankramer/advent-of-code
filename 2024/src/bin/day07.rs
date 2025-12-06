use rayon::prelude::*;

const IN: &str = include_str!("day07.txt");

fn run(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .into_par_iter()
        .map(|line| {
            let (ans, nums) = line.split_once(": ").unwrap();
            let ans: i64 = ans.parse().unwrap();
            let xs: Vec<_> = nums
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            let mut q = vec![(xs[0], true)];
            for x in &xs[1..xs.len() - 1] {
                let mut new_q = vec![];
                for (y, is_p1) in q {
                    (y + x <= ans).then(|| new_q.push((y + x, is_p1)));
                    (y * x <= ans).then(|| new_q.push((y * x, is_p1)));
                    (poor_mans_concat(y, *x) <= ans)
                        .then(|| new_q.push((poor_mans_concat(y, *x), false)));
                }
                q = new_q;
            }

            let x = xs[xs.len() - 1];
            let mut is_p2 = false;
            for (y, is_p1) in q {
                if y + x == ans {
                    is_p2 = true;
                    if is_p1 {
                        return (ans, ans);
                    }
                }

                if y * x == ans {
                    is_p2 = true;
                    if is_p1 {
                        return (ans, ans);
                    }
                }

                if poor_mans_concat(y, x) == ans {
                    is_p2 = true;
                }
            }

            if is_p2 {
                return (0, ans);
            }

            (0, 0)
        })
        .reduce(|| (0, 0), |(p1, p2), (q1, q2)| (p1 + q1, p2 + q2))
}

#[inline]
fn poor_mans_concat(a: i64, b: i64) -> i64 {
    if b < 10 {
        a * 10 + b
    } else if b < 100 {
        a * 100 + b
    } else if b < 1000 {
        a * 1000 + b
    } else {
        unimplemented!("add as necessary");
    }
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 07\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (3749, 11387));
        assert_eq!(run(IN), (2664460013123, 426214131924213));
    }
}
