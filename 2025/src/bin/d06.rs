fn run(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let width = input.lines().next().unwrap().len();
    let height = input.len() / (width + 1) - 1;

    let ops = input[input.len() - width - 1..]
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|op| match op {
            '*' => |acc, val| acc * val,
            '+' => |acc, val| acc + val,
            op => panic!("unknown op {op}"),
        })
        .collect::<Vec<_>>();

    let nums_p1 = input
        .split_whitespace()
        .filter_map(|str| str.parse().ok())
        .collect::<Vec<usize>>();

    let nums_p2 = (0..width)
        .map(|c| {
            (0..height)
                .map(|r| bytes[r * (width + 1) + c])
                .filter_map(|c| {
                    c.is_ascii_digit()
                        .then_some((c.saturating_sub(b'0')) as usize)
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (exp, digit)| acc + digit * 10usize.pow(exp as u32))
        })
        .collect::<Vec<_>>();

    let p1 = (0..ops.len())
        .map(|col| {
            (0..height)
                .map(|row| nums_p1[row * ops.len() + col])
                .reduce(ops[col])
                .unwrap()
        })
        .sum();

    let p2 = nums_p2
        .split(|&x| x == 0)
        .enumerate()
        .map(|(op_idx, nums)| {
            nums.iter()
                .map(|num| num)
                .copied()
                .reduce(ops[op_idx])
                .unwrap()
        })
        .sum();

    (p1, p2)
}

fn main() {
    assert_eq!((4277556, 3263827), run(EX));
    assert_eq!((4449991244405, 9348430857627), run(IN));

    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX: &str = include_str!("../../inputs/d06.ex");
const IN: &str = include_str!("../../inputs/d06.in");
