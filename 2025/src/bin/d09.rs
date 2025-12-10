use std::collections::BTreeMap;
use std::ops::RangeBounds;

type Point = [usize; 2];
type Line = (usize, usize);
type Edges = BTreeMap<usize, Vec<Line>>;

fn run(input: &str) -> (usize, usize) {
    let points: Vec<Point> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| [x.parse().unwrap(), y.parse().unwrap()])
        .collect();

    let lines: Vec<_> = points
        .iter()
        .copied()
        .zip(points.iter().copied().cycle().skip(1))
        .collect();

    let mut rects: Vec<(usize, Point, Point)> = points
        .iter()
        .enumerate()
        .flat_map(|(idx_a, a)| points.iter().skip(idx_a + 1).map(move |b| (a, b)))
        .map(|(&a, &b)| normalize(a, b))
        .filter(|(a, b)| b[0] > a[0] && b[1] > a[1])
        .map(|(a, b)| ((1 + b[0] - a[0]) * (1 + b[1] - a[1]), a, b))
        .collect();

    rects.sort_unstable_by(|a, b| b.cmp(a));

    let p1 = rects[0].0;

    let (edges_x, edges_y) = lines.iter().map(|&(a, b)| normalize(a, b)).fold(
        (
            BTreeMap::<usize, Vec<Line>>::new(),
            BTreeMap::<usize, Vec<Line>>::new(),
        ),
        |(mut x, mut y), (a, b)| {
            match a[0] == b[0] {
                true => x.entry(a[0]).or_default().push((a[1], b[1])),
                false => y.entry(a[1]).or_default().push((a[0], b[0])),
            };
            (x, y)
        },
    );

    for (size, top_left, bottom_right) in rects {
        let (x_min, y_min, x_max, y_max) =
            (top_left[0], top_left[1], bottom_right[0], bottom_right[1]);

        let top_left_inside = intersects(&edges_x, x_min + 1.., [y_min + 1]) % 2 == 1;
        let vertical_intersects = intersects(&edges_x, x_min + 1..x_max, [y_min + 1, y_max - 1]);
        let horizontal_intersects = intersects(&edges_y, y_min + 1..y_max, [x_min + 1, x_max - 1]);

        if top_left_inside && vertical_intersects == 0 && horizontal_intersects == 0 {
            return (p1, size);
        }
    }

    unreachable!()
}

fn normalize(a: Point, b: Point) -> (Point, Point) {
    (
        [a[0].min(b[0]), a[1].min(b[1])],
        [a[0].max(b[0]), a[1].max(b[1])],
    )
}

fn intersects<R: RangeBounds<usize>, const N: usize>(
    edges: &Edges,
    range: R,
    coords: [usize; N],
) -> usize {
    edges
        .range(range)
        .filter(|(_, lines)| {
            lines.iter().any(|l| {
                coords
                    .iter()
                    .map(|coord| (l.0..l.1).contains(coord))
                    .reduce(|a, b| a || b)
                    .unwrap()
            })
        })
        .count()
}

fn main() {
    assert_eq!(run(EX2), (300, 75));
    assert_eq!(run(EX1), (50, 24));
    assert_eq!(run(IN), (4750297200, 1578115935));
    aoc25::print(run(IN));
    aoc25::bench(|| run(IN));
}

const EX1: &str = include_str!("../../inputs/d09-1.ex");
const EX2: &str = include_str!("../../inputs/d09-2.ex");
const IN: &str = include_str!("../../inputs/d09.in");
