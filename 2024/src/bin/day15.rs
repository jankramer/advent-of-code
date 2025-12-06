use aoc24::grid::Grid;
use aoc24::input::{arrow_to_direction, Input};
use aoc24::vector::Vec2;
use itertools::Itertools;
use rustc_hash::FxHashSet;

const IN: Input = Input::new(include_str!("day15.txt"));

fn run(input: Input) -> (i64, i64) {
    let (grid_p1, moves) = input.split_once("\n\n");
    let moves: Vec<Vec2> = moves.chars().filter_map(arrow_to_direction).collect();

    let grid_p2 = grid_p1
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    '.' => ['.', '.'],
                    _ => panic!(),
                })
                .collect::<String>()
        })
        .join("\n");

    (solve(grid_p1, &moves), solve(grid_p2, &moves))
}

fn solve(grid: impl Into<Grid>, moves: &[Vec2]) -> i64 {
    let grid = grid.into();
    let cursor = grid.position(&'@').unwrap();

    let (grid, _) = moves.iter().fold((grid, cursor), step);

    grid.data
        .iter()
        .filter(|(_, &v)| v == '[' || v == 'O')
        .map(|(&pos, _)| pos.0 * 100 + pos.1)
        .sum()
}

fn step((mut grid, cursor): (Grid, Vec2), &dir: &Vec2) -> (Grid, Vec2) {
    let next_pos = cursor + dir;

    if grid[next_pos] == '#' {
        return (grid, cursor);
    }

    if grid[next_pos] == '.' {
        grid[cursor] = '.';
        grid[next_pos] = '@';
        return (grid, next_pos);
    }

    let mut movable: FxHashSet<Vec2> = FxHashSet::default();
    let mut q = vec![cursor];
    while let Some(current) = q.pop() {
        if !movable.insert(current) {
            continue;
        }

        let current_next = current + dir;
        match grid[current_next] {
            '#' => {
                return (grid, cursor);
            }
            'O' => {
                q.push(current_next);
            }
            '[' => {
                q.push(current_next);
                q.push(current_next + Vec2(0, 1));
            }
            ']' => {
                q.push(current_next);
                q.push(current_next + Vec2(0, -1));
            }
            '.' => {}
            _ => panic!(),
        }
    }

    let mut vacated = FxHashSet::default();
    let mut pending = vec![];

    for pos in movable {
        pending.push((pos + dir, grid.data.remove(&pos).unwrap()));
        vacated.insert(pos);
    }

    for (pos, val) in pending {
        vacated.remove(&pos);
        grid[pos] = val;
    }

    for pos in vacated {
        grid[pos] = '.';
    }

    (grid, cursor + dir)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 15\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#,
    );

    const T2: Input = Input::new(
        r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 2028);

        let test_input_result = run(T2);
        assert_eq!(test_input_result.0, 10092);
        assert_eq!(test_input_result.1, 9021);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 1349898);
        assert_eq!(real_input_result.1, 1376686);
    }
}
