use aoc25::matrix::Matrix;
use std::ops::Range;

fn run(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let expected_state = parts[0][1..parts[0].len() - 1].char_indices().fold(
            [false; 10],
            |mut acc, (idx, c)| {
                acc[idx] = c == '#';
                acc
            },
        );

        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected_counts = parts[parts.len() - 1]
            .trim_matches(['{', '}'])
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        p1 += solve_p1(expected_state, &buttons);
        p2 += solve_p2(&expected_counts, &buttons);
    }

    (p1, p2)
}

type State = [bool; 10];

fn solve_p1(expected: State, buttons: &[Vec<usize>]) -> usize {
    fn toggle_lights(state: State, buttons: &[usize]) -> State {
        let mut new_state = state;
        for &button in buttons {
            new_state[button] = !state[button];
        }

        new_state
    }

    let mut p1 = 0;
    let mut presses = vec![(0, State::default())];
    'outer: for _ in 0.. {
        let mut new = vec![];
        for (count, state) in presses {
            for button in buttons {
                let new_state = toggle_lights(state, button);
                if new_state == expected {
                    p1 += count + 1;
                    break 'outer;
                }

                new.push(((count + 1), toggle_lights(state, button)));
            }
        }

        presses = new;
    }

    p1
}

fn solve_p2(expected: &[usize], buttons: &[Vec<usize>]) -> usize {
    let button_max: Vec<_> = buttons
        .iter()
        .map(|btn| btn.iter().map(|&wire| expected[wire]).min().unwrap())
        .collect();

    let matrix: Matrix<192, f64> =
        Matrix::from_fn(expected.len(), buttons.len() + 1, |r, c| {
            match c < buttons.len() {
                true => buttons[c].contains(&r) as i64 as f64,
                false => expected[r] as f64,
            }
        });

    let mut matrix_rref = matrix;
    matrix_rref.rref();

    let mut bound_vars = matrix_rref.bound_variables();

    if is_fully_bound(&bound_vars) {
        return bound_vars
            .iter()
            .map(|x| x.unwrap().round() as usize)
            .sum::<usize>();
    }

    let mut search = vec![];

    while !is_fully_bound(&bound_vars) {
        let (idx, _) = bound_vars
            .iter()
            .enumerate()
            .filter(|(idx, val)| !search.contains(idx) && val.is_none())
            .map(|(idx, _)| (idx, button_max[idx]))
            .min_by_key(|(_, max)| *max)
            .unwrap();

        search.push(idx);

        let mut matrix = matrix;
        for &idx in &search {
            let row = matrix.add_row();
            row[idx] = 1.;
            row[buttons.len() - 1] = 0.;
        }

        matrix.rref();
        bound_vars = matrix.bound_variables();
    }

    let ranges: Vec<_> = search.iter().map(|&idx| 0..button_max[idx] + 1).collect();

    let mut min: usize = usize::MAX;

    cartesian_product(&ranges, |values| {
        let mut matrix = matrix;
        for (i, &button_idx) in search.iter().enumerate() {
            let row = matrix.add_row();
            row[button_idx] = 1.;
            row[buttons.len()] = values[i] as f64;
        }

        matrix.rref();
        bound_vars = matrix.bound_variables();

        if is_fully_bound(&bound_vars) {
            let mut sum = 0;
            for value in bound_vars.iter() {
                let value = value.unwrap();
                if value < -0.1 || !is_zero(&(value.round() - value)) {
                    return;
                }

                sum += value.round() as usize;
            }

            min = min.min(sum);
        }
    });

    if min == usize::MAX {
        unreachable!();
    }

    min
}

fn is_zero(x: &f64) -> bool {
    x.abs() < 0.001
}

fn is_fully_bound(values: &[Option<f64>]) -> bool {
    values.iter().all(|x| x.is_some())
}

pub fn cartesian_product(ranges: &[Range<usize>], mut f: impl FnMut(&[usize])) {
    let mut values: Vec<usize> = ranges.iter().map(|r| r.start).collect();

    'outer: loop {
        f(&values);

        for i in (0..ranges.len()).rev() {
            if values[i] < ranges[i].end - 1 {
                values[i] += 1;
                for j in i + 1..ranges.len() {
                    values[j] = ranges[j].start;
                }
                break;
            } else {
                if i == 0 {
                    break 'outer;
                } else {
                    values[i] = ranges[i].start;
                }

                continue;
            }
        }
    }
}

fn main() {
    assert_eq!(run(EX), (7, 33));
    assert_eq!(run(IN), (461, 16386));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &str = include_str!("../../inputs/d10.ex");

const IN: &str = include_str!("../../inputs/d10.in");
