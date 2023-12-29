use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, io};

use crate::pos::Pos;

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub index: usize,
    pub name: Option<String>,
    pub pos: Pos,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub j: usize,
    pub dist: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

type Mask = u64;
type Cache = HashMap<(usize, Mask), Option<usize>>;

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn generate_dot(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "graph G {{")?;
        for node in &self.nodes {
            write!(file, "  _{} [label=\"", node.index)?;
            if let Some(name) = &node.name {
                write!(file, "{name}")?;
            } else {
                write!(file, "{}", node.index)?;
            }
            writeln!(file, "\"];")?;
        }
        for node in &self.nodes {
            for edge in &node.edges {
                let other = &self.nodes[edge.j];
                if other.edges.iter().any(|e| e.j == node.index) {
                    if node.index < other.index {
                        writeln!(
                            file,
                            "  _{} -- _{} [label=\"{}\"];",
                            node.index, other.index, edge.dist
                        )?;
                    }
                } else {
                    writeln!(
                        file,
                        "  _{} -> _{} [label=\"{}\"];",
                        node.index, edge.j, edge.dist
                    )?;
                }
            }
        }
        writeln!(file, "}}")
    }

    pub fn solve(&self) -> usize {
        let mut cache = Cache::new();

        assert_eq!(self.nodes[0].name.as_deref().unwrap_or(""), "start");
        self.rec(0, 1, &mut cache).unwrap()
    }

    pub fn rec(&self, i: usize, visited: Mask, cache: &mut Cache) -> Option<usize> {
        let key = (i, visited);
        if let Some(&res) = cache.get(&key) {
            return res;
        }

        if self.is_goal(i) {
            cache.insert(key, Some(0));
            return Some(0);
        }

        let mut best: Option<usize> = None;

        for Edge { j, dist } in &self.nodes[i].edges {
            if ((visited >> j) & 1) == 0 {
                let next_visited = visited | (1 << j);
                if let Some(sub) = self.rec(*j, next_visited, cache) {
                    let sol = sub + dist;
                    if best.is_none() || best.unwrap() < sol {
                        best = Some(sol);
                    }
                }
            }
        }

        cache.insert(key, best);
        best
    }

    fn is_goal(&self, i: usize) -> bool {
        self.nodes[i].name.as_deref().unwrap_or("") == "goal"
    }
}
