use cp_library::{
    algebra::{DefaultOrdering, ReversedOrdering},
    cartesian_tree::CartesianTree,
    prelude::*,
};

type Tree = CartesianTree<ReversedOrdering<DefaultOrdering<usize>>>;

fn go(tree: &Tree, u: Option<usize>, i: usize, j: usize) -> usize {
    let Some(u) = u else {
        return 0;
    };
    assert!(i < j);
    assert!(i <= u);
    assert!(u < j);
    assert!(j <= tree.len());
    let r1 = go(tree, tree[u].left, i, u) + (j - 1 - u);
    let r2 = (u - i) + go(tree, tree[u].right, u + 1, j);
    r1.min(r2)
}

fn solve(a: Vec<usize>) -> usize {
    let tree = Tree::from_iter_with(a, Default::default());
    go(&tree, Some(tree.root()), 0, tree.len())
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n);
    let res = solve(a);
    cout.println(res);
}

pub fn main() {
    driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn case() {
        assert_eq!(solve(vec![4, 1, 3, 2, 5]), 1);
    }

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            5
            3
            1 2 3
            5
            4 1 3 2 5
            6
            4 5 3 6 2 1
            7
            6 5 1 7 4 2 3
            15
            7 4 10 12 9 14 5 3 8 11 1 15 2 13 6
"),
        @"
        0
        1
        3
        3
        9
        ")
    }
}
