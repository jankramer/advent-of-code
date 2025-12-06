fn run(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let sizes_by_length = (0..16)
        .map(|i| (1..=i / 2).filter(|j| i % j == 0).rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut digits = Vec::with_capacity(16);
    for x in input
        .split(",")
        .map(|str| str.split_once('-').unwrap())
        .flat_map(|(l, r)| l.trim().parse::<usize>().unwrap()..=r.trim().parse::<usize>().unwrap())
    {
        to_digits(x, &mut digits);

        let Some(&size) = sizes_by_length[digits.len()].iter().find(|&&size| {
            digits
                .chunks(size)
                .skip(1)
                .all(|chunk| chunk == &digits[0..size])
        }) else {
            continue;
        };

        p1 += x * (size == digits.len() / 2 && digits.len() % 2 == 0) as usize;
        p2 += x;
    }

    (p1, p2)
}

fn to_digits(mut x: usize, out: &mut Vec<u8>) {
    out.clear();

    while x > 0 {
        out.push((x % 10) as u8);
        x /= 10;
    }
}

fn main() {
    assert_eq!(run(EX), (1227775554, 4174379265));
    assert_eq!(run(IN), (18952700150, 28858486244));

    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX: &str = include_str!("../../inputs/d02.ex");
const IN: &str = include_str!("../../inputs/d02.in");
