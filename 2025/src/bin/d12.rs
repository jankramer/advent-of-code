fn run(input: &str) -> (usize, usize) {
    let mut p1 = 0;

    let mut sizes = vec![];
    for block in input.split("\n\n") {
        if block.as_bytes()[1] == b':' {
            sizes.push(block.bytes().filter(|&b| b == b'#').count());
        } else {
            for line in block.lines() {
                let (l, r) = line.split_once(": ").unwrap();
                let available: usize = l.split('x').map(num).product();
                let required: usize = r
                    .split_whitespace()
                    .map(num)
                    .enumerate()
                    .map(|(i, x)| sizes[i] * x)
                    .sum();

                if required < available {
                    p1 += 1;
                }
            }
        }
    }

    (p1, 0)
}

fn num(s: &str) -> usize {
    s.parse().unwrap()
}

fn main() {
    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const IN: &str = include_str!("../../inputs/d12.in");
