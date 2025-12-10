type State = [bool; 10];

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

        let mut presses = vec![(0, State::default())];
        'outer: for i in 0.. {
            let mut new = vec![];
            for (count, state) in presses {
                for button in &buttons {
                    let new_state = toggle_lights(state, &button);
                    if new_state == expected_state {
                        p1 += count + 1;
                        break 'outer;
                    }

                    new.push(((count + 1), toggle_lights(state, &button)));
                }
            }

            presses = new;
        }
    }

    (p1, p2)
}

fn toggle_lights(state: State, buttons: &[usize]) -> State {
    let mut new_state = state;
    for &button in buttons {
        new_state[button] = !state[button];
    }

    new_state
}

fn main() {
    assert_eq!(run(EX), (7, 0));
    // assert_eq!(run(IN), (0, 0));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &str = include_str!("../../inputs/d10.ex");
const IN: &str = include_str!("../../inputs/d10.in");
