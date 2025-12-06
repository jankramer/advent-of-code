use rustc_hash::FxHashMap;

const IN: &str = include_str!("day11.txt");

fn run(input: &str) -> (usize, usize) {
    let mut numbers: FxHashMap<usize, usize> = input
        .split_whitespace()
        .map(|x| (x.parse().unwrap(), 1))
        .collect();

    let mut p1 = 0;
    for i in 0..75 {
        if i == 25 {
            p1 = numbers.values().sum();
        }

        let mut new_numbers = FxHashMap::default();
        for (number, count) in numbers.into_iter() {
            let digits = if number < 10 { 1 } else { number.ilog10() + 1 };

            match (number, digits % 2) {
                (0, _) => {
                    *new_numbers.entry(1).or_default() += count;
                }
                (x, 0) => {
                    *new_numbers.entry(x / 10usize.pow(digits / 2)).or_default() += count;
                    *new_numbers
                        .entry(x - ((x / 10usize.pow(digits / 2)) * 10usize.pow(digits / 2)))
                        .or_default() += count;
                }
                (x, _) => {
                    *new_numbers.entry(x * 2024).or_default() += count;
                }
            }
        }

        numbers = new_numbers;
    }

    (p1, numbers.values().sum())
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 11\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}
