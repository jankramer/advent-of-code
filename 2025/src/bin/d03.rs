fn run(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let mut indices = vec![vec![]; 10];
    for l in input.lines() {
        indices.iter_mut().for_each(|xs| xs.clear());
        for (idx, c) in l.char_indices() {
            indices[c.to_digit(10).unwrap() as usize].push(idx)
        }

        p1 += max_val(l, &indices, 2);
        p2 += max_val(l, &indices, 12);
    }

    (p1, p2)
}

fn max_val(line: &str, indices: &[Vec<usize>], num_digits: usize) -> usize {
    let mut cursor_left: usize = 0;
    let mut cursor_right: usize = line.len() - num_digits;
    let mut result = 0;

    'outer: for i in 0..num_digits {
        let digits_left = num_digits - i - 1;

        for j in (0..=9).rev() {
            let Some(&idx) = indices[j]
                .iter()
                .find(|&&index| index >= cursor_left && index <= cursor_right)
            else {
                continue;
            };

            cursor_left = idx + 1;
            cursor_right += 1;
            result += j * 10_usize.pow(digits_left as u32);

            continue 'outer;
        }
    }

    result
}

fn main() {
    assert_eq!((357, 3121910778619), run(EX));
    assert_eq!((17034, 168798209663590), run(IN));

    aoc25::print(run(IN));
    aoc25::bench(|| {
        run(IN);
    });
}

const EX: &str = include_str!("../../inputs/d03.ex");
const IN: &str = include_str!("../../inputs/d03.in");
