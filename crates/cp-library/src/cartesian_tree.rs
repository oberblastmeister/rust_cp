use crate::Itertools;

#[derive(Debug, Clone)]
pub struct Node<W> {
    pub index: usize,
    pub weight: W,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

pub struct CartesianTree<W> {
    nodes: Box<[Node<W>]>,
    root: usize,
}

impl<W> std::ops::Index<usize> for CartesianTree<W> {
    type Output = Node<W>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<W> std::ops::IndexMut<usize> for CartesianTree<W> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<W> CartesianTree<W>
where
    W: Ord + Clone,
{
    pub fn new(values: &[W]) -> CartesianTree<W> {
        assert!(!values.is_empty());
        let mut nodes = values
            .into_iter()
            .cloned()
            .enumerate()
            .map(|(i, weight)| Node { index: i, weight, left: None, right: None })
            .collect_vec()
            .into_boxed_slice();
        let mut monotonic_stack: Vec<usize> = Vec::new();
        for node in 0..nodes.len() {
            let mut left = None;
            while let Some(&last) = monotonic_stack.last() {
                if nodes[last].weight <= nodes[node].weight {
                    break;
                }
                left = monotonic_stack.pop();
            }
            nodes[node].left = left;
            if let Some(&parent) = monotonic_stack.last() {
                nodes[parent].right = Some(node);
            }
            monotonic_stack.push(node);
        }
        CartesianTree { nodes, root: monotonic_stack[0] }
    }

    pub fn root(&self) -> usize {
        self.root
    }
}
