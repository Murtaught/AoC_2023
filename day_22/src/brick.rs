use std::cmp::{max, min};

use crate::pos::{Coord, Pos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brick {
    pub tl: Pos,
    pub dr: Pos,
}

impl Brick {
    pub fn new(mut tl: Pos, mut dr: Pos) -> Self {
        for coord in Coord::iter() {
            if tl.get(coord) > dr.get(coord) {
                std::mem::swap(tl.get_mut(coord), dr.get_mut(coord));
            }
        }

        Self { tl, dr }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut it = s.split('~');
        let tl = Pos::parse(it.next()?)?;
        let dr = Pos::parse(it.next()?)?;
        if it.next().is_some() {
            return None;
        }
        Some(Self::new(tl, dr))
    }

    pub fn range(&self, coord: Coord) -> (i32, i32) {
        let &a = self.tl.get(coord);
        let &b = self.dr.get(coord);
        (a, b)
    }

    pub fn intersects(&self, other: &Brick) -> bool {
        Coord::iter().all(|coord| {
            let (a, b) = self.range(coord);
            let (c, d) = other.range(coord);
            max(a, c) <= min(b, d)
        })
    }

    pub fn step(&mut self, coord: Coord, steps: i32) {
        *self.tl.get_mut(coord) += steps;
        *self.dr.get_mut(coord) += steps;
    }

    pub fn stepped(&self, coord: Coord, steps: i32) -> Brick {
        let mut brick = self.clone();
        brick.step(coord, steps);
        brick
    }
}
