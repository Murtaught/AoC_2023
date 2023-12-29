use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

pub type Id = usize;

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub map: IdMap,
    pub edges: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Default)]
pub struct IdMap {
    map: HashMap<String, Id>,
    reverse: Vec<String>,
    counter: Id,
}

impl IdMap {
    pub fn get_id(&mut self, name: &str) -> Id {
        if let Some(&id) = self.map.get(name) {
            return id;
        }

        self.counter += 1;
        self.map.insert(name.to_string(), self.counter);
        self.reverse.resize(self.counter + 1, String::new());
        self.reverse[self.counter] = name.to_string();
        self.counter
    }

    #[allow(dead_code)]
    pub fn get_name(&self, id: Id) -> Option<&str> {
        if 0 < id && id < self.reverse.len() {
            Some(&self.reverse[id])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Id)> {
        self.map.iter()
    }
}

impl Graph {
    pub fn parse(content: &str) -> Self {
        let mut map = IdMap::default();
        let mut eset = HashSet::<(usize, usize)>::new();
        for line in content.lines() {
            let mut it = line.split([':', ' ']).filter(|s| !s.is_empty());
            let name = it.next().unwrap();
            let i = map.get_id(name);
            for other in it {
                let j = map.get_id(other);
                let p = if i < j { (i, j) } else { (j, i) };
                eset.insert(p);
            }
        }

        let n = map.map.values().max().unwrap() + 1;
        let mut edges = vec![vec![false; n]; n];
        for (i, j) in eset {
            edges[i][j] = true;
            edges[j][i] = true;
        }

        Self { map, edges }
    }

    pub fn load_from_file(file_path: &str) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap();
        Self::parse(&content)
    }

    pub fn find_path(&self, start: Id, goal: Id) -> Option<Vec<Id>> {
        let mut came_from = HashMap::<Id, Id>::new();
        let mut queue = VecDeque::<Id>::new();
        queue.push_back(start);
        while let Some(cur) = queue.pop_front() {
            if cur == goal {
                let mut i = goal;
                let mut res = Vec::new();
                while i != start {
                    res.push(i);
                    i = came_from.get(&i).copied().unwrap();
                }
                res.push(start);
                res.reverse();
                return Some(res);
            }

            let it = self.edges[cur]
                .iter()
                .enumerate()
                .filter(|(_, e)| **e)
                .map(|(i, _)| i);

            for i in it {
                if let Entry::Vacant(e) = came_from.entry(i) {
                    e.insert(cur);
                    queue.push_back(i);
                }
            }
        }

        None
    }

    pub fn scan(&self, start: Id) -> (HashSet<Id>, HashSet<Id>) {
        let mut graph = self.clone();
        let max_node_id = self.map.map.values().copied().max().unwrap();

        let mut ours = HashSet::<Id>::new();
        let mut theirs = HashSet::<Id>::new();

        ours.insert(start);

        for goal in 1..=max_node_id {
            if goal == start {
                continue;
            }

            #[cfg(debug)]
            eprintln!(
                "Probing pair {:?} -- {:?} ...",
                self.map.get_name(start),
                self.map.get_name(goal)
            );

            // Restoring edges to their initial state.
            graph.edges = self.edges.clone();

            let mut unique_paths = 0;

            while let Some(path) = graph.find_path(start, goal) {
                unique_paths += 1;

                if unique_paths > 3 {
                    // We don't really care about exact number.
                    break;
                }

                // Deleting the path.
                let pairs = path.iter().copied().zip(path.iter().copied().skip(1));
                for (i, j) in pairs {
                    graph.edges[i][j] = false;
                    graph.edges[j][i] = false;
                }
            }

            assert!(unique_paths >= 3);

            if unique_paths > 3 {
                ours.insert(goal);
            } else {
                theirs.insert(goal);
            }

        }

        (ours, theirs)
    }
}
