use std::cmp::Ordering;

pub const IN: &str = include_str!("day02.txt");

pub fn run(input: &str) -> (usize, usize) {
    let lines: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    (
        lines.iter().filter(|x| is_safe_a(x)).count(),
        lines.iter().filter(|x| is_safe_b(x)).count(),
    )
}

fn is_safe_a(numbers: &[usize]) -> bool {
    let direction = numbers[1].cmp(&numbers[0]);
    if direction == Ordering::Equal {
        return false;
    }

    for i in 0..numbers.len() - 1 {
        if numbers[i + 1].cmp(&numbers[i]) != direction {
            return false;
        }

        if numbers[i].abs_diff(numbers[i + 1]) > 3 {
            return false;
        }
    }

    true
}

fn is_safe_b(numbers: &[usize]) -> bool {
    if is_safe_a(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let (left, right) = numbers.split_at(i);
        let alt: Vec<_> = left.iter().chain(right.iter().skip(1)).copied().collect();

        if is_safe_a(&alt) {
            return true;
        }
    }

    false
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

    const T1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (2, 4));
        assert_eq!(run(IN), (486, 540));
    }
}
