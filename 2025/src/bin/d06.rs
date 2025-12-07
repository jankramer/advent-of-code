fn run(input: &[u8]) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let line_width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let (nums, ops) = input.split_at(input.len() - line_width);

    let height = nums.len() / line_width;

    let mut offset = 0;
    while offset < line_width {
        let op = match ops[offset] {
            b'*' => |acc, x| acc * x,
            b'+' => |acc, x| acc + x,
            _ => unreachable!(),
        };

        let next = ops[offset + 1..]
            .iter()
            .position(|&b| !b.is_ascii_whitespace());

        let width = next.unwrap_or(ops.len() - offset - 1);

        p1 += (0..height)
            .map(|y| {
                nums[offset + y * line_width..offset + y * line_width + width]
                    .iter()
                    .fold(0, from_digits)
            })
            .reduce(op)
            .unwrap();

        p2 += (0..width)
            .map(|x| {
                input
                    .iter()
                    .skip(offset + x)
                    .step_by(line_width)
                    .fold(0, from_digits)
            })
            .reduce(op)
            .unwrap();

        offset += width + 1;
    }

    (p1, p2)
}

fn from_digits(acc: usize, b: &u8) -> usize {
    match b {
        b'0'..=b'9' => acc * 10 + (b - b'0') as usize,
        _ => acc,
    }
}

fn main() {
    assert_eq!((4277556, 3263827), run(EX));
    assert_eq!((4449991244405, 9348430857627), run(IN));

    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX: &[u8] = include_bytes!("../../inputs/d06.ex");
const IN: &[u8] = include_bytes!("../../inputs/d06.in");
