#[derive(Clone, Debug)]
pub struct Dsu {
    pub parent: Box<[usize]>,
    pub size: Box<[usize]>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n].into_boxed_slice(),
        }
    }

    pub fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    pub fn merge(&mut self, u: usize, v: usize) {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return;
        }
        if self.size[u] < self.size[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.parent[v] = u;
        self.size[u] += self.size[v];
    }
}
