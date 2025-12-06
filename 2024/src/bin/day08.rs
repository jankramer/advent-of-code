const IN: &[u8] = include_bytes!("day08.txt");

fn run(input: &[u8]) -> (usize, usize) {
    let width: i64 = input.iter().position(|&c| c == b'\n').unwrap() as i64;
    let height: i64 = input.trim_ascii_end().len() as i64 / width;

    let mut map: [Vec<(i64, i64)>; 128] = [const { Vec::new() }; 128];
    for (idx, &char) in input.iter().filter(|&&c| c != b'\n').enumerate() {
        if char == b'.' {
            continue;
        }

        map[char as usize].push((idx as i64 % width, idx as i64 / width));
    }

    let mut p1 = vec![false; (width * height + 1) as usize];
    let mut p2 = vec![false; (width * height + 1) as usize];

    let insert = |bitmap: &mut Vec<bool>, p: &(i64, i64)| {
        if p.0 >= 0 && p.1 >= 0 && p.0 < height && p.1 < width {
            bitmap[(p.1 + p.0 * width) as usize] = true;
            true
        } else {
            false
        }
    };

    for indices in &map {
        for a in indices {
            for b in indices {
                if a == b {
                    continue;
                }

                p2[(a.1 + a.0 * width) as usize] = true;

                let dr = b.0 - a.0;
                let dc = b.1 - a.1;

                if insert(&mut p1, &(a.0 - dr, a.1 - dc)) {
                    for i in 1.. {
                        if !insert(&mut p2, &(a.0 - i * dr, a.1 - i * dc)) {
                            break;
                        };
                    }
                }
            }
        }
    }

    (
        p1.iter().map(|&x| x as usize).sum(),
        p2.iter().map(|&x| x as usize).sum(),
    )
}

fn main() {
    let (p1, p2) = run(IN);

    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        run(IN);
    }
    let elapsed = now.elapsed();
    println!("{}ns\n", elapsed.as_nanos() / 100000);
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &[u8] = b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test() {
        assert_eq!(run(T1), (14, 34));
        assert_eq!(run(IN), (369, 1169));
    }
}
