fn run(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let mut grid = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c == '@'))
        .collect::<Vec<_>>();

    let width = input.lines().next().unwrap().len() as i64;
    let height = grid.len() as i64 / width;

    let mut remove = vec![];
    let mut queue = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| val.then_some(idx))
        .collect::<Vec<_>>();

    let mut new_queue = vec![];

    loop {
        while let Some(idx) = queue.pop() {
            match nb8(width, height, idx as i64)
                .filter(|&nb| grid[nb])
                .count()
                < 4
            {
                true => remove.push(idx),
                false => new_queue.push(idx),
            }
        }

        if remove.is_empty() {
            break;
        }

        p1 += remove.len() * (p1 == 0) as usize;
        p2 += remove.len();

        remove.drain(..).for_each(|idx| grid[idx] = false);
        queue.extend(new_queue.drain(..));
    }

    (p1, p2)
}

const NB8: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

fn nb8(width: i64, height: i64, idx: i64) -> impl Iterator<Item = usize> {
    let (x, y) = (idx % width, idx / width);

    NB8.into_iter()
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .map(move |(dx, dy)| (x + dx, y + dy))
        .filter_map(move |(x, y)| {
            if x < 0 || x >= width || y < 0 || y >= height {
                None
            } else {
                Some((x + y * width) as usize)
            }
        })
}

fn main() {
    assert_eq!(run(EX), (13, 43));
    assert_eq!(run(IN), (1491, 8722));

    aoc25::print(run(IN));
    aoc25::bench(|| std::hint::black_box(run(IN)));
}

const EX: &str = include_str!("../../inputs/d04.ex");
const IN: &str = include_str!("../../inputs/d04.in");
