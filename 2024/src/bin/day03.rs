use regex::Regex;

const IN: &str = include_str!("day03.txt");

fn run(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    let mut enabled = true;
    for instr in parse(input) {
        match instr {
            Instr::Do => {
                enabled = true;
            }
            Instr::Dont => {
                enabled = false;
            }
            Instr::Mul(l, r) => {
                p1 += l * r;
                if enabled {
                    p2 += l * r;
                }
            }
        }
    }

    (p1, p2)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

pub fn parse(input: &str) -> Vec<Instr> {
    let regex =
        Regex::new(r#"(?<mul>mul)\((?<a>\d+),(?<b>\d+)\)|(?<do>do)\(\)|(?<dont>don't\(\))"#)
            .unwrap();

    regex
        .captures_iter(input)
        .map(|cap| {
            if cap.name("mul").is_some() {
                return Instr::Mul(
                    cap.name("a").unwrap().as_str().parse().unwrap(),
                    cap.name("b").unwrap().as_str().parse().unwrap(),
                );
            }

            if cap.name("do").is_some() {
                return Instr::Do;
            }

            if cap.name("dont").is_some() {
                return Instr::Dont;
            }

            panic!("unknown instr: {cap:#?}");
        })
        .collect()
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 03\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (161, 48));
        assert_eq!(run(IN), (185797128, 89798695));
    }
}
