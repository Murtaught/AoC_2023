use instr::{Instr, ParseMode};
use line_set::LineSet;

mod dir;
mod instr;
mod line;
mod pos;
mod line_set;

fn main() {
    let content = std::fs::read_to_string("input").unwrap();
    let instrs_1 = parse_instrs(&content, ParseMode::Normal);
    let instrs_2 = parse_instrs(&content, ParseMode::Color);

    println!("solve (p1): {}", solve(&instrs_1));
    println!("solve (p2): {}", solve(&instrs_2));
}

fn parse_instrs(content: &str, mode: ParseMode) -> Vec<Instr> {
    content
        .lines()
        .map(|line| Instr::parse(line, mode).unwrap())
        .collect()
}

fn solve(instrs: &[Instr]) -> u64 {
    let set = LineSet::new(instrs);
    set.compute_area()
}
