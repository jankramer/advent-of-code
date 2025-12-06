use aoc24::input::Input;
use itertools::Itertools;

const IN: Input = Input::new(include_str!("day13.txt"));

fn run(input: Input) -> (i64, i64) {
    let (mut p1, mut p2) = (0, 0);

    for (ax, ay, bx, by, x, y) in input.numbers().tuples() {
        p1 += solve(ax, ay, bx, by, x, y);
        p2 += solve(ax, ay, bx, by, x + 10000000000000, y + 10000000000000);
    }

    (p1, p2)
}

fn solve(ax: i64, ay: i64, bx: i64, by: i64, x: i64, y: i64) -> i64 {
    let b = (y * ax - x * ay) / (ax * by - bx * ay);
    let a = (x - bx * b) / ax;

    if a * ax + b * bx != x || a * ay + b * by != y {
        return 0;
    }

    a * 3 + b
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

    const T1: Input = Input::new(
        r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 480);
    }
}
