const IN: &str = include_str!("day04.txt");

fn run(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let (mut p1, mut p2) = (0, 0);

    let dirs = vec![N, NE, E, SE, S, SW, W, NW];

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'X' {
                for dir in &dirs {
                    let mut valid = true;
                    for i in 1..=3 {
                        if !grid
                            .get(((y as isize) + i * dir[1]) as usize)
                            .and_then(|row| row.get(((x as isize) + i * dir[0]) as usize))
                            .is_some_and(|c| c == &XMAS[i as usize])
                        {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        p1 += 1;
                    }
                }
            }

            if c == 'A' {
                let diag_1: String = [-1, 0, 1]
                    .into_iter()
                    .map(|n: isize| [x as isize + n, y as isize + n])
                    .filter_map(|xy| {
                        (xy[0] >= 0 && xy[1] >= 0).then_some([xy[0] as usize, xy[1] as usize])
                    })
                    .filter_map(|xy| grid.get(xy[1]).and_then(|row| row.get(xy[0])))
                    .collect();

                let diag_2: String = [-1, 0, 1]
                    .into_iter()
                    .map(|n| [x as isize + -n, y as isize + n])
                    .filter_map(|xy| {
                        (xy[0] >= 0 && xy[1] >= 0).then_some([xy[0] as usize, xy[1] as usize])
                    })
                    .filter_map(|xy| grid.get(xy[1]).and_then(|row| row.get(xy[0])))
                    .collect();

                if (&diag_1 == "MAS" || &diag_1 == "SAM") && (&diag_2 == "MAS" || &diag_2 == "SAM")
                {
                    p2 += 1;
                }
            }
        }
    }

    (p1, p2)
}

const N: [isize; 2] = [0, -1];
const NE: [isize; 2] = [1, -1];
const E: [isize; 2] = [1, 0];
const SE: [isize; 2] = [1, 1];
const S: [isize; 2] = [0, 1];
const SW: [isize; 2] = [-1, 1];
const W: [isize; 2] = [-1, 0];
const NW: [isize; 2] = [-1, -1];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 06\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (18, 9));
        assert_eq!(run(IN), (2642, 1974));
    }
}
