use geometry::coords::Coords;
use itertools::Itertools;
use std::collections::btree_map::Entry::Vacant;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part A: {}", solve_a(INPUT));
    println!("Part B: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let mut state: State = State {
        elves: input
            .lines()
            .enumerate()
            .flat_map(|(y, xs)| {
                xs.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Coords([x as isize, y as isize])),
                    _ => None,
                })
            })
            .collect(),
    };

    for i in 0..10 {
        state.tick(i);
    }

    let bounding_box = state.bounding_box();
    let mut num_empty = 0;
    for y in bounding_box.0 .0[1]..=bounding_box.1 .0[1] {
        for x in bounding_box.0 .0[0]..=bounding_box.1 .0[0] {
            if !state.elves.contains(&Coords([x, y])) {
                num_empty += 1;
            }
        }
    }

    num_empty
}

fn solve_b(input: &str) -> usize {
    let mut state: State = State {
        elves: input
            .lines()
            .enumerate()
            .flat_map(|(y, xs)| {
                xs.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Coords([x as isize, y as isize])),
                    _ => None,
                })
            })
            .collect(),
    };

    let mut prev_elves = state.elves.clone();
    for i in 0.. {
        state.tick(i);
        if state.elves == prev_elves {
            return i + 1;
        }

        prev_elves = state.elves.clone();
    }

    return 0;
    //
    // for i in 0..10 {
    //     state.tick(i);
    //     println!("== End of Round {}\n", i + 1);
    //     println!("{state}");
    // }
    //
    // let bounding_box = state.bounding_box();
    // let mut num_empty = 0;
    // for y in bounding_box.0 .0[1]..=bounding_box.1 .0[1] {
    //     for x in bounding_box.0 .0[0]..=bounding_box.1 .0[0] {
    //         if !state.elves.contains(&Coords([x, y])) {
    //             num_empty += 1;
    //         }
    //     }
    // }
    //
    // num_empty
}

struct State {
    elves: BTreeSet<Coords<2>>,
}

impl State {
    fn bounding_box(&self) -> (Coords<2>, Coords<2>) {
        let xs = self.elves.iter().map(|p| p.0[0]).collect_vec();
        let ys = self.elves.iter().map(|p| p.0[1]).collect_vec();

        (
            Coords([
                xs.iter().min().unwrap().clone(),
                ys.iter().min().unwrap().clone(),
            ]),
            Coords([
                xs.iter().max().unwrap().clone(),
                ys.iter().max().unwrap().clone(),
            ]),
        )
    }

    fn tick(&mut self, round: usize) {
        let directions = [
            ([0, -1], [[-1, -1], [0, -1], [1, -1]]),
            ([0, 1], [[-1, 1], [0, 1], [1, 1]]),
            ([-1, 0], [[-1, -1], [-1, 0], [-1, 1]]),
            ([1, 0], [[1, -1], [1, 0], [1, 1]]),
        ];

        let mut proposed_moves: BTreeMap<Coords<2>, Coords<2>> = BTreeMap::new();
        let mut blocked_tiles: BTreeSet<Coords<2>> = BTreeSet::new();

        for elf in self.elves.iter() {
            if !elf.neighbours().any(|p| self.elves.contains(&p)) {
                continue;
            }

            let move_to = directions
                .iter()
                .cycle()
                .skip(round)
                .take(4)
                .find(|(_, neighbours)| {
                    !neighbours
                        .iter()
                        .filter(|&p| !blocked_tiles.contains(&Coords(*p)))
                        .any(|p| self.elves.contains(&(*elf + Coords(*p))))
                })
                .map(|(dir, _)| *elf + Coords(*dir));

            if let Some(m) = move_to {
                if let Vacant(e) = proposed_moves.entry(m) {
                    e.insert(*elf);
                } else {
                    proposed_moves.remove(&m);
                    blocked_tiles.insert(m);
                }
            }
        }

        for (to, from) in proposed_moves {
            self.elves.remove(&from);
            self.elves.insert(to);
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bounding_box = self.bounding_box();

        for y in bounding_box.0 .0[1]..=bounding_box.1 .0[1] {
            for x in bounding_box.0 .0[0]..=bounding_box.1 .0[0] {
                if self.elves.contains(&Coords([x, y])) {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        Ok(())
    }
}
