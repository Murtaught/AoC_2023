use std::collections::VecDeque;
use std::fmt;

use crate::node::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal {
    pub src: Id,
    pub dest: Id,
    pub high: bool,
}

pub type Queue = VecDeque<Signal>;

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.src,
            if self.high { "HIGH" } else { "low" },
            self.dest
        )
    }
}

impl Node {
    pub fn send_signal(&mut self, sig: Signal, queue: &mut Queue) {
        assert_eq!(self.id, sig.dest);

        // if !sig.high && INTERESTING_NODES.iter().any(|&nm| self.name == nm) {
        // eprintln!("Node \"{}\" triggered at button press {button_press}!", self.name);
        // }

        let high = match &mut self.tp {
            NodeType::Repeater => Some(sig.high),
            NodeType::FlipFlop(state) => {
                if sig.high {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            NodeType::Conjunction(states) => {
                states.insert(sig.src, sig.high);
                Some(!states.values().all(|&v| v))
            }
        };

        if let Some(high) = high {
            for &dest in &self.dests {
                queue.push_back(Signal {
                    src: self.id,
                    dest,
                    high,
                })
            }
        }
    }
}
