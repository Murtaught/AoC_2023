use crate::pos::Pos;

#[derive(Debug)]
pub struct Field {
    n: usize,
    m: usize,
    fld: Vec<Vec<u8>>,
}

impl Field {
    pub fn parse(content: &str) -> Field {
        let fld: Vec<Vec<u8>> = content
            .lines()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();

        let n = fld.len();
        let m = fld.first().unwrap().len();
        assert!(!fld.iter().any(|row| row.len() != m));

        Field { n, m, fld }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn m(&self) -> usize {
        self.m
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.i >= 0 && pos.j >= 0 && (pos.i as usize) < self.n && (pos.j as usize) < self.m
    }

    pub fn get(&self, pos: Pos) -> u64 {
        debug_assert!(self.contains(pos));
        let i = pos.i as usize;
        let j = pos.j as usize;
        self.fld[i][j] as u64
    }
}
