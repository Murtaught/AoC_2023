use std::collections::HashMap;

use crate::{
    dir::Dir,
    instr::Instr,
    line::{Line, LineIntersection},
    pos::Pos,
};

#[derive(Debug)]
pub struct LineSet {
    lines: Vec<Line>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Corner {
    Pipe,
    TL,
    TR,
    DL,
    DR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pin {
    j: i64,
    corner: Corner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Out,
    E1,
    In,
    E2,
}

impl LineSet {
    pub fn new(instrs: &[Instr]) -> LineSet {
        let mut cur = Pos::default();
        let mut lines = Vec::new();
        for instr in instrs {
            let next = cur.step(instr.dir, instr.steps);
            let line = Line::new(cur, next);
            lines.push(line);
            cur = next;
        }
        LineSet { lines }
    }

    pub fn bounding_box(&self) -> (Pos, Pos) {
        let mut min_i = i64::MAX;
        let mut min_j = i64::MAX;
        let mut max_i = i64::MIN;
        let mut max_j = i64::MIN;
        for line in &self.lines {
            for p in [line.a, line.b] {
                min_i = min_i.min(p.i);
                min_j = min_j.min(p.j);
                max_i = max_i.max(p.i);
                max_j = max_j.max(p.j);
            }
        }
        (Pos::new(min_i, min_j), Pos::new(max_i, max_j))
    }

    pub fn find_intersections(&self) -> Vec<LineIntersection> {
        let mut res = Vec::new();

        for i in 0..self.lines.len() {
            let l1 = &self.lines[i];
            for j in 0..i {
                let l2 = &self.lines[j];
                if let Some(intersection) = l1.intersect(l2) {
                    match intersection {
                        LineIntersection::Middle(_) | LineIntersection::Segment(_) => {
                            res.push(intersection);
                        }
                        _ => {}
                    }
                }
            }
        }
        res
    }

    pub fn compute_area(&self) -> u64 {
        assert_eq!(self.find_intersections(), vec![]);
        let (tl, dr) = self.bounding_box();

        // Let's prepare to scan the field vertically.
        // It's easy for horizontal lines.
        let mut h_lines = HashMap::<i64, Vec<Line>>::new();
        for &line in &self.lines {
            if line.is_horizontal() {
                h_lines.entry(line.a.i).or_default().push(line);
            }
        }

        // A bit harder for vertical lines - we'll use sorted vector.
        let v_lines: Vec<Line> = self
            .lines
            .iter()
            .copied()
            .filter(Line::is_vertical)
            .collect();

        let empty_vec = Vec::new();
        let mut area = 0_u64;

        #[cfg(debug)]
        const MASK: u32 = (1 << 19) - 1;
        for i in tl.i..=dr.i {
            #[cfg(debug)]
            if ((i as u32) & MASK) == 0 {
                eprintln!(
                    "{:.2}%",
                    (i - tl.i) as f64 / (dr.i - tl.i + 1) as f64 * 100.0
                );
            }

            let mut pins = Vec::<Pin>::with_capacity(1024);

            let h_lines = h_lines.get(&i).unwrap_or(&empty_vec);

            for v_line in &v_lines {
                if v_line.intersects_row(i) {
                    let mut added_something = false;

                    for p in [v_line.a, v_line.b] {
                        if p.i == i {
                            let corner = find_corner(
                                p,
                                if p == v_line.a { Dir::U } else { Dir::D },
                                h_lines,
                            )
                            .unwrap();

                            pins.push(Pin { j: p.j, corner });

                            added_something = true;
                            break;
                        }
                    }

                    if !added_something {
                        pins.push(Pin {
                            j: v_line.a.j,
                            corner: Corner::Pipe,
                        });
                    }
                }
            }

            pins.sort();
            // eprintln!("i = {i}: {pins:?}");

            let mut row_area = 0_u64;
            let mut beginning: Option<i64> = None;
            let mut state = State::Out;
            let mut enter: Option<Corner> = None;
            for &pin in &pins {
                let corner = pin.corner;

                if state != State::Out {
                    if let Some(prev) = beginning {
                        row_area += (pin.j - prev) as u64;
                        beginning = Some(pin.j);
                    }
                }

                match state {
                    State::Out => match corner {
                        Corner::Pipe => {
                            state = State::In;
                            beginning = Some(pin.j);
                        }
                        Corner::TL | Corner::DL => {
                            state = State::E1;
                            enter = Some(corner);
                            beginning = Some(pin.j);
                        }
                        _ => panic!("Unexpected combination of {state:?} and {corner:?}."),
                    },

                    State::E1 => match corner {
                        Corner::TR => {
                            state = match enter {
                                Some(Corner::DL) => State::In,
                                Some(Corner::TL) => State::Out,
                                _ => panic!("Unexpected enter state (E1 TR): {enter:?}"),
                            };
                            enter = None;
                        }
                        Corner::DR => {
                            state = match enter {
                                Some(Corner::TL) => State::In,
                                Some(Corner::DL) => State::Out,
                                _ => panic!("Unexpected enter state (E1 DR): {enter:?}"),
                            };
                            enter = None;
                        }
                        _ => panic!("Unexpected combination of {state:?} and {corner:?}."),
                    },

                    State::In => match corner {
                        Corner::Pipe => {
                            state = State::Out;
                        }
                        Corner::TL | Corner::DL => {
                            state = State::E2;
                            enter = Some(corner);
                        }
                        _ => panic!("Unexpected combination of {state:?} and {corner:?}."),
                    },

                    State::E2 => match corner {
                        Corner::TR => {
                            state = match enter {
                                Some(Corner::DL) => State::Out,
                                Some(Corner::TL) => State::In,
                                _ => panic!("Unexpected enter state (E2 TR): {enter:?}"),
                            };
                            enter = None;
                        }
                        Corner::DR => {
                            state = match enter {
                                Some(Corner::TL) => State::Out,
                                Some(Corner::DL) => State::In,
                                _ => panic!("Unexpected enter state (E2 DR): {enter:?}"),
                            };
                            enter = None;
                        }
                        _ => panic!("Unexpected combination of {state:?} and {corner:?}."),
                    },
                }

                if state == State::Out {
                    if let Some(prev) = beginning {
                        row_area += (pin.j - prev + 1) as u64;
                        beginning = None;
                    }
                }
            }

            area += row_area;
        }

        area
    }
}

fn find_corner(p: Pos, d1: Dir, h_lines: &[Line]) -> Option<Corner> {
    let mut d2: Option<Dir> = None;
    for line in h_lines {
        if p == line.a {
            d2 = Some(Dir::L);
            break;
        } else if p == line.b {
            d2 = Some(Dir::R);
            break;
        }
    }

    let d2 = d2?;
    match (d1, d2) {
        (Dir::U, Dir::L) => Some(Corner::TL),
        (Dir::U, Dir::R) => Some(Corner::TR),
        (Dir::D, Dir::L) => Some(Corner::DL),
        (Dir::D, Dir::R) => Some(Corner::DR),
        _ => None,
    }
}
