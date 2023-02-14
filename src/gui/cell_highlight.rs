use std::collections::HashSet;

pub struct Highlight(HashSet<(usize, usize)>);

impl Highlight {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn insert(&mut self, val: (usize, usize)) {
        self.0.insert(val);
    }
    pub fn contains(&self, k: &(usize, usize)) -> bool {
        self.0.contains(k)
    }
}
