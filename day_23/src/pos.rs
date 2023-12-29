use std::fmt::{self, Debug};

use crate::dir::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos {
    pub i: i32,
    pub j: i32,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Pos {
    pub fn new<T>(i: T, j: T) -> Self
    where
        T: TryInto<i32>,
        T::Error: Debug,
    {
        Self {
            i: i.try_into().unwrap(),
            j: j.try_into().unwrap(),
        }
    }

    pub fn step(mut self, dir: Dir, count: i32) -> Self {
        debug_assert!(count > 0);
        match dir {
            Dir::U => self.i -= count,
            Dir::R => self.j += count,
            Dir::D => self.i += count,
            Dir::L => self.j -= count,
        }
        self
    }

    pub fn neighborhood(self) -> impl Iterator<Item = Pos> {
        Dir::iter().map(move |dir| self.step(dir, 1))
    }
}

pub fn dir(a: Pos, b: Pos) -> Option<Dir> {
    if a.i == b.i {
        if a.j + 1 == b.j {
            Some(Dir::R)
        } else if b.j + 1 == a.j {
            Some(Dir::L)
        } else {
            None
        }
    } else if a.j == b.j {
        if a.i + 1 == b.i {
            Some(Dir::D)
        } else if b.i + 1 == a.i {
            Some(Dir::U)
        } else {
            None
        }
    } else {
        None
    }
}
