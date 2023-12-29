use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    U,
    R,
    D,
    L,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Dir> {
        [Dir::U, Dir::R, Dir::D, Dir::L].into_iter()
    }

    pub fn as_char(self) -> char {
        match self {
            Dir::U => 'U',
            Dir::R => 'R',
            Dir::D => 'D',
            Dir::L => 'L',
        }
    }
}
