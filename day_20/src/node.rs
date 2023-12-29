use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Repeater,
    FlipFlop(bool),
    Conjunction(HashMap<Id, bool>),
}

pub type Id = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: Id,
    pub name: String,
    pub tp: NodeType,
    pub dests: Vec<Id>,
    pub sources: Vec<Id>,
}

#[derive(Debug, Clone, Default)]
pub struct IdMap {
    map: HashMap<String, Id>,
    counter: Id,
}

impl IdMap {
    pub fn get_id(&mut self, name: &str) -> Id {
        if let Some(&id) = self.map.get(name) {
            return id;
        }

        self.counter += 1;
        self.map.insert(name.to_string(), self.counter);
        self.counter
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Id)> {
        self.map.iter()
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: 0,
            name: String::new(),
            tp: NodeType::Repeater,
            dests: Vec::new(),
            sources: Vec::new(),
        }
    }
    
    pub fn parse(line: &str, map: &mut IdMap) -> Option<Node> {
        let mut it = line.split(['-', '>', ',', ' ']).filter(|s| !s.is_empty());

        let name = it.next()?;
        let tp = NodeType::from_name(name);
        let name = name.strip_prefix(['&', '%']).unwrap_or(name).to_string();
        let id = map.get_id(&name);
        let connections: Vec<Id> = it.map(|s| map.get_id(s)).collect();

        Some(Node {
            id,
            name,
            tp,
            dests: connections,
            sources: Vec::new(),
        })
    }
}

impl NodeType {
    pub fn from_name(s: &str) -> Self {
        match s.chars().next() {
            Some('&') => Self::Conjunction(HashMap::new()),
            Some('%') => Self::FlipFlop(false),
            _ => Self::Repeater,
        }
    }
}
