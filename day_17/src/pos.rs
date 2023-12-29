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
        T::Error: Debug
    {
        Self {
            i: i.try_into().unwrap(),
            j: j.try_into().unwrap(),
        }
    }

    pub fn step(mut self, dir: Dir) -> Self {
        match dir {
            Dir::N => self.i -= 1,
            Dir::E => self.j += 1,
            Dir::S => self.i += 1,
            Dir::W => self.j -= 1,
        }
        self
    }
}
