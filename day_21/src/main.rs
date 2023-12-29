use crate::field::Field;

mod field;
mod pos;

const STEPS: usize = 26_501_365;

fn main() {
    let field = Field::load_from_file("input");
    println!(
        "ans (p1): {}",
        field.bfs(field.start(), 64).last().copied().unwrap()
    );

    let (n, m) = field.size();
    assert_eq!(n, 131);
    assert_eq!(m, 131);
    assert_eq!(STEPS, 202300 * n + 65);

    let div = STEPS / n;
    let rem = STEPS % n;
    let xs = field.bfs(field.start(), n * 2 + rem);

    let p0 = xs[rem];
    let p1 = xs[rem + n] - p0;
    let p2 = xs[rem + 2 * n] - p1 - p0;
    let ans = p0 + (p1 * div) + (div * (div - 1) / 2) * (p2 - p1);

    // eprintln!("polynomial: {p0} {p1} {p2}");
    println!("ans (p2): {ans}");
}
