use itertools::Itertools;

const IN: &str = include_str!("day01.txt");

fn run(input: &str) -> (usize, usize) {
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .tuples()
        .unzip();

    xs.sort();
    ys.sort();

    let map = ys.iter().counts();

    let (mut p1, mut p2) = (0, 0);
    for (x, y) in xs.iter().zip(&ys) {
        p1 += x.abs_diff(*y);
        p2 += x * map.get(x).unwrap_or(&0);
    }

    (p1, p2)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (11, 31));
        assert_eq!(run(IN), (2378066, 18934359));
    }
}
