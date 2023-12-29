use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Dir> {
        [Dir::N, Dir::E, Dir::S, Dir::W].into_iter()
    }

    pub fn as_char(self) -> char {
        match self {
            Dir::N => 'N',
            Dir::E => 'E',
            Dir::S => 'S',
            Dir::W => 'W',
        }
    }

    pub fn reverse(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}
