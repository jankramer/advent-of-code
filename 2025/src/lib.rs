use std::fmt::Display;

pub fn print<P1: Display, P2: Display>((p1, p2): (P1, P2)) {
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

pub fn bench<F: Fn() -> T, T>(func: F) {
    let now = std::time::Instant::now();
    func();
    let nanos = now.elapsed().as_nanos();
    if nanos < 100 {
        return;
    }

    let iters = 2500000000 / nanos;

    let now = std::time::Instant::now();
    for _ in 0..iters {
        func();
    }

    let prefixes = ["ns", "μs", "ms", "s"];
    let duration = now.elapsed().as_nanos() / iters;
    let idx = (duration.ilog10().saturating_sub(1) / 3).clamp(0, prefixes.len() as u32 - 1);

    println!(
        "{}{}",
        duration / 10_u128.pow(idx * 3),
        prefixes[idx as usize]
    );
}
