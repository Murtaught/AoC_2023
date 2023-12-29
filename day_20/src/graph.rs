use crate::{node::*, signal::*};
use std::{cmp::max, collections::{HashMap, HashSet}, io::{self, Write}, fs::File};

#[derive(Debug, Clone)]
pub struct Graph {
    pub map: IdMap,
    pub nodes: Vec<Node>,
    pub signals_fired: [usize; 2],
}

pub type IdSet = HashSet<Id>;

impl Graph {
    pub fn parse(content: &str) -> Option<Self> {
        let mut map = IdMap::default();
        let mut nodes = Vec::new();

        for line in content.lines() {
            let node = Node::parse(line, &mut map)?;
            let id = node.id;
            nodes.resize(max(nodes.len(), id + 1), Node::new());
            nodes[id] = node;
        }

        Some(Graph {
            map,
            nodes,
            signals_fired: [0, 0],
        })
    }

    pub fn add_node(&mut self, line: &str) {
        let node = Node::parse(line, &mut self.map).unwrap();
        let id = node.id;
        self.nodes
            .resize(max(self.nodes.len(), id + 1), Node::new());
        self.nodes[id] = node;
    }

    pub fn prepare(&mut self) {
        {
            let mut sources = HashMap::<Id, Vec<Id>>::new();

            for src_node in &self.nodes {
                let src = src_node.id;
                for &dest in &src_node.dests {
                    sources.entry(dest).or_default().push(src);
                }
            }

            for dest_node in &mut self.nodes {
                if let Some(sources) = sources.get_mut(&dest_node.id) {
                    dest_node.sources = std::mem::take(sources);
                }
            }
        }

        for node in &mut self.nodes {
            if let NodeType::Conjunction(states) = &mut node.tp {
                for &src in &node.sources {
                    states.insert(src, false);
                }
            }
        }

        for (name, &id) in self.map.iter() {
            let node = &mut self.nodes[id];
            if node.name.is_empty() {
                node.id = id;
                node.name = name.clone();
                // Everything else is already OK.
            }
        }
    }

    pub fn generate_dot(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir = LR;")?;

        for (i, node) in self.nodes.iter().enumerate() {
            if node.name.is_empty() {
                assert_eq!(i, 0);
                assert_eq!(node.id, 0);
                continue;
            }

            let id = node.id;
            let name = match &node.tp {
                NodeType::Repeater => node.name.clone(),
                NodeType::FlipFlop(_) => format!("%{}", node.name),
                NodeType::Conjunction(_) => format!("&{}", node.name),
            };
            let shape = match &node.tp {
                NodeType::Repeater => "oval",
                NodeType::FlipFlop(_) => "triangle",
                NodeType::Conjunction(_) => "box",
            };

            writeln!(file, "  _{id} [label=\"{name}\", shape=\"{shape}\"];")?;
        }

        for node in &self.nodes {
            let src = node.id;
            for &dest in &node.dests {
                writeln!(file, "  _{src} -> _{dest};")?;
            }
        }

        writeln!(file, "}}")
    }

    pub fn simulate(&mut self) -> IdSet {
        let start_id = self.map.get_id("button");
        let mut queue = Queue::with_capacity(1024);

        self.nodes[start_id].send_signal(
            Signal {
                src: 0,
                dest: start_id,
                high: false,
            },
            &mut queue
        );

        let mut triggered = IdSet::new();
        triggered.insert(start_id);

        while let Some(sig) = queue.pop_front() {
            let dest_node = &mut self.nodes[sig.dest];
            self.signals_fired[sig.high as usize] += 1;
            if !sig.high {
                triggered.insert(sig.dest);
            }
            dest_node.send_signal(sig, &mut queue);
        }

        triggered
    }
}
