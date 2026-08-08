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

#[cfg(test)]
mod tests {
    use super::Dsu;

    #[test]
    fn merges_sets_by_size() {
        let mut dsu = Dsu::new(5);

        dsu.merge(0, 1);
        dsu.merge(2, 3);
        dsu.merge(1, 3);

        let root = dsu.find(0);
        assert_eq!(dsu.find(1), root);
        assert_eq!(dsu.find(2), root);
        assert_eq!(dsu.find(3), root);
        assert_ne!(dsu.find(4), root);
        assert_eq!(dsu.size[root], 4);
    }

    #[test]
    fn merging_an_existing_set_is_a_no_op() {
        let mut dsu = Dsu::new(2);

        dsu.merge(0, 1);
        dsu.merge(1, 0);

        let root = dsu.find(0);
        assert_eq!(dsu.find(1), root);
        assert_eq!(dsu.size[root], 2);
    }
}
