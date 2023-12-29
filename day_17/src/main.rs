use std::fs;

use field::Field;
use pos::Pos;
use state::dijkstra;

use crate::state::Mode;

mod dir;
mod field;
mod pos;
mod state;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let field = Field::parse(&content);

    let start = Pos::default();
    let goal = Pos::new(field.n() - 1, field.m() - 1);

    println!("ans (p1): {}", dijkstra(&field, start, goal, Mode::Normal).unwrap());
    println!("ans (p2): {}", dijkstra(&field, start, goal, Mode::SuperCrucible).unwrap());
}
