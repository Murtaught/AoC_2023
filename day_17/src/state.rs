use crate::{dir::Dir, field::Field, pos::Pos};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FullPosition {
    pub pos: Pos,
    pub dir: Option<Dir>,
    pub steps: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct State {
    pub pos: FullPosition,
    pub cost: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    SuperCrucible,
}

impl fmt::Display for FullPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}, {}, {}}}",
            self.pos,
            self.dir.map(Dir::as_char).unwrap_or('_'),
            self.steps
        )
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} | {})", self.pos, self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    pub fn possible_next_states(&self, field: &Field, mode: Mode) -> Vec<State> {
        let rev_d = self.pos.dir.map(Dir::reverse);
        Dir::iter()
            .filter(|&d| Some(d) != rev_d)
            .filter(|&d| match mode {
                Mode::Normal => Some(d) != self.pos.dir || self.pos.steps < 3,
                Mode::SuperCrucible => {
                    if self.pos.steps < 4 {
                        self.pos.dir.is_none() || d == self.pos.dir.unwrap()
                    } else {
                        Some(d) != self.pos.dir || self.pos.steps < 10
                    }
                }
            })
            .map(|d| (d, self.pos.pos.step(d)))
            .filter(|&(_, p)| field.contains(p))
            .map(|(d, p)| State {
                pos: FullPosition {
                    pos: p,
                    dir: Some(d),
                    steps: if Some(d) == self.pos.dir {
                        self.pos.steps + 1
                    } else {
                        1
                    },
                },
                cost: self.cost + field.get(p),
            })
            .collect()
    }
}

pub fn dijkstra(field: &Field, start: Pos, goal: Pos, mode: Mode) -> Option<u64> {
    let mut dist = HashMap::<FullPosition, u64>::new();
    let mut heap = BinaryHeap::<State>::new();

    let starting_state = State {
        pos: FullPosition {
            pos: start,
            dir: None,
            steps: 0,
        },
        cost: 0,
    };

    dist.insert(starting_state.pos, 0);
    heap.push(starting_state);

    while let Some(cur) = heap.pop() {
        if cur.pos.pos == goal {
            return Some(cur.cost);
        }

        // We may have already found a better way.
        if let Some(&best_known) = dist.get(&cur.pos) {
            if cur.cost > best_known {
                continue;
            }
        }

        for next in cur.possible_next_states(field, mode) {
            let best_known = dist.get(&next.pos).copied().unwrap_or(u64::MAX);
            if next.cost < best_known {
                dist.insert(next.pos, next.cost);
                heap.push(next);
            }
        }
    }

    // What am I doing wrong?
    // Debug output:

    None
}
