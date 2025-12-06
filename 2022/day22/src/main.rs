use itertools::Itertools;
use parse_display::Display;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let layout = vec![
        Tile {
            top_left: (50, 0),
            bottom_right: (99, 49),
            adjacent: vec![
                (1, Facing::E, false),
                (2, Facing::S, false),
                (4, Facing::E, true),
                (5, Facing::E, false),
            ],
        },
        Tile {
            top_left: (100, 0),
            bottom_right: (149, 49),
            adjacent: vec![
                (3, Facing::W, true),
                (2, Facing::W, false),
                (0, Facing::W, false),
                (5, Facing::N, false),
            ],
        },
        Tile {
            top_left: (50, 50),
            bottom_right: (99, 99),
            adjacent: vec![
                (1, Facing::N, false),
                (3, Facing::S, false),
                (4, Facing::S, false),
                (0, Facing::N, false),
            ],
        },
        Tile {
            top_left: (50, 100),
            bottom_right: (99, 149),
            adjacent: vec![
                (1, Facing::W, true),
                (5, Facing::W, false),
                (4, Facing::W, false),
                (2, Facing::N, false),
            ],
        },
        Tile {
            top_left: (0, 100),
            bottom_right: (49, 149),
            adjacent: vec![
                (3, Facing::E, false),
                (5, Facing::S, false),
                (0, Facing::E, true),
                (2, Facing::E, false),
            ],
        },
        Tile {
            top_left: (0, 150),
            bottom_right: (49, 199),
            adjacent: vec![
                (3, Facing::N, false),
                (1, Facing::S, false),
                (0, Facing::S, false),
                (4, Facing::N, false),
            ],
        },
    ];

    println!("Part A: {}", solve_a(INPUT));
    println!("Part B: {}", solve_b(INPUT, layout));
}

fn solve_b(input: &str, layout: Vec<Tile>) -> isize {
    let (grid, instr) = input.split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(false),
                    '.' => Some(true),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let instr = instr
        .split_inclusive(&['R', 'L'])
        .flat_map(|p| {
            let mut xs = vec![Instr::Move(p[0..p.len() - 1].parse::<isize>().unwrap())];

            let foo = match p.chars().last() {
                Some('R') => Some(Dir::R),
                Some('L') => Some(Dir::L),
                _ => None,
            };

            if let Some(dir) = foo {
                xs.push(Instr::Rotate(dir));
            }

            xs
        })
        .collect_vec();

    let mut current_pos: (isize, isize) = (
        grid[0].iter().position(|&x| x == Some(true)).unwrap() as isize,
        0,
    );
    let mut current_facing = Facing::E;
    let mut current_tile = 0;

    for current_instr in instr {
        match current_instr {
            Instr::Move(n) => {
                for _ in 0..n {
                    let next_pos = (
                        current_pos.0 + current_facing.dir().0,
                        current_pos.1 + current_facing.dir().1,
                    );
                    if layout[current_tile].contains(next_pos) {
                        match grid
                            .get(next_pos.1 as usize)
                            .map(|xs| xs.get(next_pos.0 as usize))
                        {
                            Some(Some(Some(true))) => {
                                current_pos = next_pos;
                            }
                            Some(Some(Some(false))) => {
                                break;
                            }
                            _ => panic!("out of bounds"),
                        }
                    } else {
                        let (next_tile, next_facing, next_pos) =
                            position_in_adjacent_tile(&layout, current_pos, &current_facing);

                        match grid
                            .get(next_pos.1 as usize)
                            .map(|xs| xs.get(next_pos.0 as usize))
                        {
                            Some(Some(Some(true))) => {
                                current_pos = next_pos;
                                current_tile = next_tile;
                                current_facing = next_facing;
                            }
                            Some(Some(Some(false))) => {
                                break;
                            }
                            xxx => panic!("unexpected {:#?}", xxx),
                        }
                    }
                }
            }
            Instr::Rotate(dir) => {
                current_facing = match (current_facing, dir) {
                    (Facing::E, Dir::R) => Facing::S,
                    (Facing::E, Dir::L) => Facing::N,
                    (Facing::S, Dir::R) => Facing::W,
                    (Facing::S, Dir::L) => Facing::E,
                    (Facing::W, Dir::R) => Facing::N,
                    (Facing::W, Dir::L) => Facing::S,
                    (Facing::N, Dir::R) => Facing::E,
                    (Facing::N, Dir::L) => Facing::W,
                };
            }
        }
    }

    1000 * (current_pos.1 + 1) + (4 * (current_pos.0 + 1)) + current_facing.value() as isize
}

#[derive(Debug, Clone)]
enum Instr {
    Move(isize),
    Rotate(Dir),
}

#[derive(Debug, Clone, Display)]
enum Dir {
    R,
    L,
}

#[derive(Debug, Clone)]
enum Facing {
    E,
    S,
    W,
    N,
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Facing::E => 0,
            Facing::S => 1,
            Facing::W => 2,
            Facing::N => 3,
        }
    }

    fn dir(&self) -> (isize, isize) {
        match self {
            Facing::E => (1, 0),
            Facing::S => (0, 1),
            Facing::W => (-1, 0),
            Facing::N => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    top_left: (isize, isize),
    bottom_right: (isize, isize),
    adjacent: Vec<(usize, Facing, bool)>,
}

impl Tile {
    fn contains(&self, point: (isize, isize)) -> bool {
        point.0 >= self.top_left.0
            && point.0 <= self.bottom_right.0
            && point.1 >= self.top_left.1
            && point.1 <= self.bottom_right.1
    }
}

fn solve_a(input: &str) -> isize {
    let (grid, instr) = input.split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(false),
                    '.' => Some(true),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let instr = instr
        .split_inclusive(&['R', 'L'])
        .flat_map(|p| {
            let mut xs = vec![Instr::Move(p[0..p.len() - 1].parse::<isize>().unwrap())];

            let foo = match p.chars().last() {
                Some('R') => Some(Dir::R),
                Some('L') => Some(Dir::L),
                _ => None,
            };

            if let Some(dir) = foo {
                xs.push(Instr::Rotate(dir));
            }

            xs
        })
        .collect_vec();

    let mut current_pos: (isize, isize) = (
        grid[0].iter().position(|&x| x == Some(true)).unwrap() as isize,
        0,
    );
    let mut current_dir: (isize, isize) = (1, 0);

    for current_instr in instr {
        match current_instr {
            Instr::Move(n) => {
                for _ in 0..n {
                    let next_tile = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);

                    match grid
                        .get(next_tile.1 as usize)
                        .map(|xs| xs.get(next_tile.0 as usize))
                    {
                        Some(Some(Some(true))) => {
                            current_pos = next_tile;
                        }
                        Some(Some(Some(false))) => {
                            break;
                        }
                        _ => {
                            let wrapped_tile = match current_dir {
                                (1, _) => (
                                    grid.get(current_pos.1 as usize)
                                        .map(|xs| {
                                            xs.iter().position(|x| x.is_some()).unwrap() as isize
                                        })
                                        .unwrap(),
                                    current_pos.1,
                                ),
                                (-1, _) => (
                                    grid.get(current_pos.1 as usize)
                                        .map(|xs| {
                                            xs.iter().rposition(|x| x.is_some()).unwrap() as isize
                                        })
                                        .unwrap(),
                                    current_pos.1,
                                ),
                                (_, 1) => (
                                    current_pos.0,
                                    grid.iter()
                                        .position(|xs| {
                                            xs.get(current_pos.0 as usize)
                                                .map(|x| x.is_some())
                                                .unwrap_or(false)
                                        })
                                        .unwrap() as isize,
                                ),
                                (_, -1) => (
                                    current_pos.0,
                                    grid.iter()
                                        .rposition(|xs| {
                                            xs.get(current_pos.0 as usize)
                                                .map(|x| x.is_some())
                                                .unwrap_or(false)
                                        })
                                        .unwrap() as isize,
                                ),
                                _ => panic!("unable to find wrapped tile: invalid direction"),
                            };

                            match grid
                                .get(wrapped_tile.1 as usize)
                                .map(|xs| xs.get(wrapped_tile.0 as usize))
                            {
                                Some(Some(Some(true))) => {
                                    current_pos = wrapped_tile;
                                }
                                Some(Some(Some(false))) => {
                                    break;
                                }
                                x => panic!("invalid wrapped tile: {:#?} {:#?}", x, wrapped_tile),
                            }
                        }
                    }
                }
            }
            Instr::Rotate(dir) => {
                current_dir = match (current_dir, dir) {
                    ((1, 0), Dir::R) => (0, 1),
                    ((0, 1), Dir::R) => (-1, 0),
                    ((-1, 0), Dir::R) => (0, -1),
                    ((0, -1), Dir::R) => (1, 0),

                    ((1, 0), Dir::L) => (0, -1),
                    ((0, -1), Dir::L) => (-1, 0),
                    ((-1, 0), Dir::L) => (0, 1),
                    ((0, 1), Dir::L) => (1, 0),

                    _ => panic!("invalid direction"),
                }
            }
        }
    }

    1000 * (current_pos.1 + 1)
        + (4 * (current_pos.0 + 1))
        + match current_dir {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            dir => panic!("invalid dir {:#?}", dir),
        }
}

fn position_in_adjacent_tile(
    layout: &Vec<Tile>,
    current_pos: (isize, isize),
    current_facing: &Facing,
) -> (usize, Facing, (isize, isize)) {
    let current_tile = layout.iter().find(|t| t.contains(current_pos)).unwrap();
    let (next_tile_i, next_facing, invert) = &current_tile.adjacent[current_facing.value()];

    let dx = if *invert {
        current_tile.bottom_right.0 - current_pos.0
    } else {
        current_pos.0 - current_tile.top_left.0
    };

    let dy = if *invert {
        current_tile.bottom_right.1 - current_pos.1
    } else {
        current_pos.1 - current_tile.top_left.1
    };

    let next_tile = &layout[*next_tile_i];
    (
        *next_tile_i,
        next_facing.clone(),
        match (current_facing, next_facing) {
            (Facing::E, Facing::E) => (next_tile.top_left.0, next_tile.top_left.1 + dy),
            (Facing::E, Facing::S) => (next_tile.top_left.0 + dy, next_tile.top_left.1),
            (Facing::E, Facing::W) => (next_tile.bottom_right.0, next_tile.top_left.1 + dy),
            (Facing::E, Facing::N) => (next_tile.top_left.0 + dy, next_tile.bottom_right.1),

            (Facing::S, Facing::E) => (next_tile.top_left.0, next_tile.top_left.1 + dx),
            (Facing::S, Facing::S) => (next_tile.top_left.0 + dx, next_tile.top_left.1),
            (Facing::S, Facing::W) => (next_tile.bottom_right.0, next_tile.top_left.1 + dx),
            (Facing::S, Facing::N) => (next_tile.top_left.0 + dx, next_tile.bottom_right.1),

            (Facing::W, Facing::E) => (next_tile.top_left.0, next_tile.top_left.1 + dy),
            (Facing::W, Facing::S) => (next_tile.top_left.0 + dy, next_tile.top_left.1),
            (Facing::W, Facing::W) => (next_tile.bottom_right.0, next_tile.top_left.1 + dy),
            (Facing::W, Facing::N) => (next_tile.top_left.0 + dy, next_tile.bottom_right.1),

            (Facing::N, Facing::E) => (next_tile.top_left.0, next_tile.top_left.1 + dx),
            (Facing::N, Facing::S) => (next_tile.top_left.0 + dx, next_tile.top_left.1),
            (Facing::N, Facing::W) => (next_tile.bottom_right.0, next_tile.top_left.1 + dx),
            (Facing::N, Facing::N) => (next_tile.top_left.0 + dx, next_tile.bottom_right.1),
        },
    )
}
