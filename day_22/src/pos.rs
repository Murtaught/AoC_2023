use std::fmt::{self, Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Coord {
    X,
    Y,
    Z,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Pos {
    pub fn new<T>(x: T, y: T, z: T) -> Self
    where
        T: TryInto<i32>,
        T::Error: Debug,
    {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            z: z.try_into().unwrap(),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut it = s.split(',');
        let x = it.next()?.parse().ok()?;
        let y = it.next()?.parse().ok()?;
        let z = it.next()?.parse().ok()?;
        if it.next().is_some() {
            return None;
        }
        Some(Self { x, y, z })
    }

    pub fn get(&self, coord: Coord) -> &i32 {
        match coord {
            Coord::X => &self.x,
            Coord::Y => &self.y,
            Coord::Z => &self.z,
        }
    }

    pub fn get_mut(&mut self, coord: Coord) -> &mut i32 {
        match coord {
            Coord::X => &mut self.x,
            Coord::Y => &mut self.y,
            Coord::Z => &mut self.z,
        }
    }
}

impl Coord {
    pub fn iter() -> impl Iterator<Item = Coord> {
        use Coord::*;
        [X, Y, Z].into_iter()
    }
}
