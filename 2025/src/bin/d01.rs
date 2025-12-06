fn run(input: &str) -> (usize, i64) {
    let (mut p1, mut p2, mut x) = (0, 0, 50);

    for l in input.lines() {
        let delta = l[1..].parse::<i64>().unwrap() * if &l[0..1] == "L" { -1 } else { 1 };
        let x_new = (x + delta).rem_euclid(100);
        let count = ((x + delta) / 100).abs();

        p1 += (x_new == 0) as usize;
        p2 += count + (x != 0 && (x + delta) <= 0) as i64;

        x = x_new;
    }

    (p1, p2)
}

fn main() {
    assert_eq!((3, 6), run(EX));
    assert_eq!((1168, 7199), run(IN));

    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX: &str = include_str!("../../inputs/d01.ex");
const IN: &str = include_str!("../../inputs/d01.in");
