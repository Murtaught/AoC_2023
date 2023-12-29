use std::fmt::{self, Debug};

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

    pub fn neighborhood(&self) -> impl Iterator<Item = Pos> {
        let i = self.i;
        let j = self.j;
        [
            Pos { i: i - 1, j },
            Pos { i, j: j + 1 },
            Pos { i: i + 1, j },
            Pos { i, j: j - 1 },
        ]
        .into_iter()
    }
}
