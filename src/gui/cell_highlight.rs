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
    pub fn clear(&mut self) {
        self.cells.clear()
    }
    pub fn insert(&mut self, val: (usize, usize)) {
        self.cells.insert(val);
    }
    pub fn contains(&self, k: &(usize, usize)) -> bool {
        self.cells.contains(k)
    }
}
