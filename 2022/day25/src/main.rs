use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input.test.txt");

fn main() {
    assert_eq!(snafu_to_dec("2=-01"), 976);
    assert_eq!(dec_to_snafu(20), "1-0");
    assert_eq!(solve(INPUT_TEST), "2=-1=0");

    println!("Part A: {}", solve(INPUT));
}

fn solve(input: &str) -> String {
    dec_to_snafu(
        input
            .lines()
            .map(|l| {
                let x = snafu_to_dec(l);
                return x;
            })
            .sum(),
    )
}

fn dec_to_snafu(input: isize) -> String {
    let mut current = "1".to_string();
    for _ in 0.. {
        let next = format!("{}=", current);

        if snafu_to_dec(&next) > input {
            break;
        }

        current = next;
    }

    let mut current = current.clone().chars().collect_vec();
    for pos in 0..current.len() {
        loop {
            let mut next = current.clone();
            match next[pos] {
                '=' => {
                    next[pos] = '-';
                }
                '-' => {
                    next[pos] = '0';
                }
                '0' => {
                    next[pos] = '1';
                }
                '1' => {
                    next[pos] = '2';
                }
                '2' => {
                    break;
                }
                _ => {
                    panic!("unexpected");
                }
            }

            if snafu_to_dec(&String::from_iter(next.clone())) > input {
                break;
            }

            current = next;
        }
    }

    String::from_iter(current)
}

//    -2  -   2
//     3  -  12
//    13  -  62
//    63

fn snafu_to_dec(input: &str) -> isize {
    let mut total = 0;
    for (pos, char) in input.chars().rev().enumerate() {
        if let Some(digit) = snafu_char_to_dec(char) {
            total += 5_isize.pow(pos as u32) * digit;
        }
    }

    total
}

fn snafu_char_to_dec(input: char) -> Option<isize> {
    match input {
        '2' => Some(2),
        '1' => Some(1),
        '0' => Some(0),
        '-' => Some(-1),
        '=' => Some(-2),
        _ => None,
    }
}
//
// 00 => 00
// 01 => 01
// 02 => 02
// 03 => 1=
// 04 => 1-
// 05 => 10
// 06 => 11
// 07 => 12
// 08 => 2=
// 09 => 2-
// 10 => 20
//
//
// 2, 1, 0, -, =
