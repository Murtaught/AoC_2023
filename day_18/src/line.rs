use crate::pos::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub a: Pos,
    pub b: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineIntersection {
    End(Pos),
    Middle(Pos),
    Segment(Line),
}

impl Line {
    pub fn new(mut a: Pos, mut b: Pos) -> Line {
        assert!(a.i == b.i || a.j == b.j);

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        Line { a, b }
    }

    pub fn is_horizontal(&self) -> bool {
        self.a.i == self.b.i
    }

    pub fn is_vertical(&self) -> bool {
        self.a.j == self.b.j
    }

    pub fn intersects_row(&self, row: i64) -> bool {
        debug_assert!(self.a.i <= self.b.i);
        self.a.i <= row && row <= self.b.i
    }

    pub fn contains(&self, p: Pos) -> bool {
        if self.is_horizontal() {
            self.a.i == p.i && self.a.j <= p.j && p.j <= self.b.j
        } else {
            self.a.j == p.j && self.a.i <= p.i && p.i <= self.b.i
        }
    }

    pub fn intersect(&self, other: &Line) -> Option<LineIntersection> {
        if self.is_horizontal() {
            let i = self.a.i;
            let j1 = self.a.j;
            let j2 = self.b.j;

            if other.is_horizontal() {
                if other.a.i != i {
                    return None;
                }

                let l = other.a.j.max(j1);
                let r = other.b.j.min(j2);
                if r < l {
                    return None;
                }

                return Some(LineIntersection::Segment(Line {
                    a: Pos::new(i, l),
                    b: Pos::new(i, r),
                }));
            }

            assert!(other.is_vertical());
            let j = other.a.j;
            let p = Pos::new(i, j);

            if !self.contains(p) || !other.contains(p) {
                return None;
            }

            if p == self.a || p == self.b {
                Some(LineIntersection::End(p))
            } else {
                Some(LineIntersection::Middle(p))
            }
        } else {
            assert!(self.is_vertical());
            let j = self.a.j;
            let i1 = self.a.i;
            let i2 = self.b.i;

            if other.is_vertical() {
                if other.a.j != j {
                    return None;
                }

                let t = other.a.i.max(i1);
                let b = other.b.i.min(i2);
                if b < t {
                    return None;
                }

                Some(LineIntersection::Segment(Line {
                    a: Pos::new(t, j),
                    b: Pos::new(b, j),
                }))
            } else {
                assert!(other.is_horizontal());
                let i = other.a.i;
                let p = Pos::new(i, j);

                if !self.contains(p) || !other.contains(p)  {
                    return None;
                }

                if p == self.a || p == self.b {
                    Some(LineIntersection::End(p))
                } else {
                    Some(LineIntersection::Middle(p))
                }
            }
        }
    }
}
