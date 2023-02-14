use std::collections::HashSet;
use crate::solution::UnionFind;

pub struct Highlight {
    cells: HashSet<(usize, usize)>,
    uf: UnionFind<(usize, usize)>,
}

impl Highlight {
    pub fn new(uf: UnionFind<(usize, usize)>) -> Self {
        Self { cells: HashSet::new(), uf }
    }
    pub fn highlight(&mut self, val: (usize, usize)) -> Option<(usize, usize)> {
        for pair in &self.cells {
            if self.uf.find(&val) == self.uf.find(pair) {
                self.cells.clear();
                return None;
            }
            break;
        }
        self.cells.clear();
        let groups = self.uf.groups();
        let id = self.uf.find(&val);
        for &entry in groups.get(&id).unwrap() {
            self.cells.insert(entry);
        }
        Some(id)
    }
    pub fn contains(&self, k: &(usize, usize)) -> bool {
        self.cells.contains(k)
    }
}
