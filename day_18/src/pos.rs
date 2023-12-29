use std::fmt::{self, Debug};

use crate::dir::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos {
    pub i: i64,
    pub j: i64,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Pos {
    pub fn new<T>(i: T, j: T) -> Self
    where
        T: TryInto<i64>,
        T::Error: Debug
    {
        Self {
            i: i.try_into().unwrap(),
            j: j.try_into().unwrap(),
        }
    }

    pub fn step(mut self, dir: Dir, count: u64) -> Self {
        debug_assert!(count > 0);
        let count = count as i64;
        match dir {
            Dir::U => self.i -= count ,
            Dir::R => self.j += count,
            Dir::D => self.i += count,
            Dir::L => self.j -= count,
        }
        self
    }
}
