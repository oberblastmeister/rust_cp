use crate::{
    Itertools,
    algebra::{DefaultOrdering, Ordering},
};

#[derive(Debug, Clone, Copy)]
pub struct Node<W> {
    pub index: usize,
    pub weight: W,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

pub struct CartesianTree<O: Ordering> {
    nodes: Box<[Node<O::T>]>,
    root: usize,
}

impl<O: Ordering> std::ops::Index<usize> for CartesianTree<O> {
    type Output = Node<O::T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<O: Ordering> std::ops::IndexMut<usize> for CartesianTree<O> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<O: Ordering> CartesianTree<O>
where
    O::T: Clone,
{
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn from_iter_with<I: IntoIterator<Item = O::T>>(values: I, algebra: O) -> CartesianTree<O> {
        let mut nodes = values
            .into_iter()
            .enumerate()
            .map(|(index, weight)| Node { index, weight, left: None, right: None })
            .collect_vec()
            .into_boxed_slice();
        assert!(!nodes.is_empty(), "cannot construct a Cartesian tree from an empty iterator");

        let mut monotonic_stack: Vec<usize> = Vec::new();
        for node in 0..nodes.len() {
            let mut left = None;
            while let Some(&last) = monotonic_stack.last() {
                if algebra.compare(nodes[last].weight.clone(), nodes[node].weight.clone()).is_le() {
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

impl<T> FromIterator<T> for CartesianTree<DefaultOrdering<T>>
where
    T: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self::from_iter_with(values, DefaultOrdering::new())
    }
}
