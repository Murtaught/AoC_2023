use crate::{
    dir::Dir,
    graph::{Edge, Graph, Node},
    pos::{dir, Pos},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Slope(Dir),
}

#[derive(Debug, Clone)]
pub struct Field {
    n: usize,
    m: usize,
    rows: Vec<Vec<Cell>>,
    start: Pos,
    goal: Pos,
}

impl Cell {
    pub fn parse(c: char) -> Option<Cell> {
        use Cell::*;
        use Dir::*;
        match c {
            '.' => Some(Empty),
            '#' => Some(Wall),
            '^' => Some(Slope(U)),
            '>' => Some(Slope(R)),
            'v' => Some(Slope(D)),
            '<' => Some(Slope(L)),
            _ => None,
        }
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Cell::Empty | Cell::Slope(_) => true,
            Cell::Wall => false,
        }
    }
}

impl Field {
    pub fn parse(content: &str) -> Self {
        let rows: Vec<Vec<Cell>> = content
            .lines()
            .map(|line| line.chars().map(|c| Cell::parse(c).unwrap()).collect())
            .collect();

        let n = rows.len();
        let m = rows.first().unwrap().len();
        assert!(rows.iter().all(|row| row.len() == m));

        let start_j = rows
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, c)| c.is_passable())
            .map(|(j, _)| j)
            .unwrap();

        let goal_j = rows
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, c)| c.is_passable())
            .map(|(j, _)| j)
            .unwrap();

        Self {
            n,
            m,
            rows,
            start: Pos::new(0, start_j),
            goal: Pos::new(n - 1, goal_j),
        }
    }

    pub fn load_from_file(file_path: &str) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap();
        Self::parse(&content)
    }

    pub fn contains(&self, pos: &Pos) -> bool {
        let n = self.n as i32;
        let m = self.m as i32;
        pos.i >= 0 && pos.j >= 0 && pos.i < n && pos.j < m
    }

    pub fn get(&self, pos: &Pos) -> &Cell {
        debug_assert!(self.contains(pos));
        let i = pos.i as usize;
        let j = pos.j as usize;
        &self.rows[i][j]
    }

    pub fn neighborhood(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        pos.neighborhood()
            .filter(|p| self.contains(p))
            .filter(move |p| match self.get(p) {
                Cell::Empty => true,
                Cell::Wall => false,
                Cell::Slope(sld) => sld == &dir(pos, *p).unwrap(),
            })
    }

    pub fn go(&self, mut cur: Pos, mut prev: Pos) -> (Pos, usize) {
        let mut steps = 1_usize;
        loop {
            let mut it = self.neighborhood(cur).filter(|p| p != &prev);

            if let Some(next) = it.next() {
                if it.next().is_some() {
                    // Reached the next fork.
                    return (cur, steps);
                } else {
                    // Only one way to go forward.
                    prev = cur;
                    cur = next;
                    steps += 1;
                }
            } else {
                // Hmmm... A path to nowhere?
                return (cur, steps);
            }
        }
    }

    pub fn to_graph(&self) -> Graph {
        let mut graph = Graph::new();
        self.visit(self.start, &mut graph);
        graph
    }

    fn visit(&self, pos: Pos, graph: &mut Graph) -> usize {
        for (i, node) in graph.nodes.iter().enumerate() {
            if node.pos == pos {
                // Already visited.
                return i;
            }
        }

        let index = graph.nodes.len();

        let mut node = Node {
            index,
            pos,
            ..Node::default()
        };

        if pos == self.start {
            node.name = Some("start".to_string());
        } else if pos == self.goal {
            node.name = Some("goal".to_string());
        };

        graph.nodes.push(node);

        for dir in self.neighborhood(pos) {
            let (next, dist) = self.go(dir, pos);
            let j = self.visit(next, graph);
            graph.nodes[index].edges.push(Edge { j, dist });
        }

        index
    }

    pub fn remove_slopes(&mut self) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                if matches!(cell, Cell::Slope(_)) {
                    *cell = Cell::Empty;
                }
            }
        }
    }
}
