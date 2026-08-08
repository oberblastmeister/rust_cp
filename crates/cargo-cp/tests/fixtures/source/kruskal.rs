use cp_library::minimum_spanning_forest;

fn main() {
    let mut edges = [(0, 1, 4), (1, 2, 2), (0, 2, 3)];
    assert_eq!(minimum_spanning_forest(3, &mut edges), 5);
}
