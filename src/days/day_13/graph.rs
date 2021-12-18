use crate::days::day_13::path::Path;
use std::collections::HashMap;

pub struct Graph {
    names: Vec<String>,
    paths: HashMap<usize, HashMap<usize, i32>>,
}

impl Graph {
    pub(crate) fn nb_guests(&self) -> usize {
        self.names.len()
    }

    pub(crate) fn get_neighbor(&self, guest_idx: usize) -> Option<&HashMap<usize, i32>> {
        self.paths.get(&guest_idx)
    }

    pub(crate) fn get_happiness(&self, guest_idx: usize, neighbor_idx: usize) -> i32 {
        let g_to_n = self
            .paths
            .get(&guest_idx)
            .and_then(|h| h.get(&neighbor_idx))
            .unwrap();
        let n_to_g = self
            .paths
            .get(&neighbor_idx)
            .and_then(|h| h.get(&guest_idx))
            .unwrap();
        g_to_n + n_to_g
    }
}

#[derive(Default)]
struct GraphAcc {
    names: Vec<String>,
    name_indices: HashMap<String, usize>,
    paths: HashMap<usize, HashMap<usize, i32>>,
}

impl GraphAcc {
    fn push_path(&mut self, path: &Path) {
        let name_idx = self.get_name_index(path.name());
        let neighbor_idx = self.get_name_index(path.neighbor());

        self.paths
            .entry(name_idx)
            .or_insert_with(HashMap::new)
            .insert(neighbor_idx, path.happiness());
    }

    fn get_name_index(&mut self, name: &str) -> usize {
        let index = self.name_indices.get(name);
        if let Some(index) = index {
            return *index;
        };

        let index = self.names.len();

        self.name_indices.insert(name.to_string(), index);
        self.names.push(name.to_string());

        index
    }

    fn build(self) -> Graph {
        Graph {
            names: self.names,
            paths: self.paths,
        }
    }

    fn add_myself(mut self) -> Self {
        let me = self.get_name_index("me");

        self.paths.insert(me, HashMap::new());

        for guest in 0..me {
            self.paths.get_mut(&guest).unwrap().insert(me, 0);
            self.paths.get_mut(&me).unwrap().insert(guest, 0);
        }
        self
    }
}

impl Graph {
    pub fn new(paths: &[Path]) -> Self {
        paths
            .iter()
            .fold(GraphAcc::default(), |mut g, p| {
                g.push_path(p);
                g
            })
            .build()
    }

    pub fn new_with_myself(paths: &[Path]) -> Self {
        paths
            .iter()
            .fold(GraphAcc::default(), |mut g, p| {
                g.push_path(p);
                g
            })
            .add_myself()
            .build()
    }
}
