fn run(input: &[u8]) -> (usize, usize) {
    let mut p1 = 0;
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start = input.iter().position(|&c| c == b'S').unwrap();

    let mut counters = vec![0; width];
    counters[start] = 1;

    for line in input.chunks(width).step_by(2) {
        for (index, byte) in line.iter().enumerate() {
            if matches!(byte, b'^') && counters[index] > 0 {
                counters[index - 1] += counters[index];
                counters[index + 1] += counters[index];
                counters[index] = 0;

                p1 += 1;
            }
        }
    }

    (p1, counters.into_iter().sum())
}

fn main() {
    assert_eq!(run(EX), (21, 40));
    assert_eq!(run(IN), (1535, 4404709551015));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &[u8] = include_bytes!("../../inputs/d07.ex");
const IN: &[u8] = include_bytes!("../../inputs/d07.in");
