use aoc24::input::Input;
use rustc_hash::FxHashMap;

const IN: Input = Input::new(include_str!("day19.txt"));

fn run(input: Input) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let (left, designs) = input.split_once("\n\n");
    let patterns: Vec<_> = left.split(", ").collect();

    let mut cache = FxHashMap::default();
    for design in designs.lines() {
        let n = count(&mut cache, &patterns, design);
        if n > 0 {
            p1 += 1;
            p2 += n;
        }
    }

    (p1, p2)
}

fn count<'a>(cache: &mut FxHashMap<&'a str, usize>, patterns: &[&str], tail: &'a str) -> usize {
    if tail.is_empty() {
        return 1;
    }

    if cache.contains_key(tail) {
        return cache[tail];
    }

    let n = patterns
        .iter()
        .filter(|&&p| tail.starts_with(p))
        .map(|&p| count(cache, patterns, &tail[p.len()..]))
        .sum();

    cache.insert(tail, n);
    n
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 19\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 6);
        assert_eq!(test_input_result.1, 16);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 293);
        assert_eq!(real_input_result.1, 623924810770264);
    }
}
