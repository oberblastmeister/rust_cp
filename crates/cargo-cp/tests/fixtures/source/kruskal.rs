use cp_library::{algebra::DefaultMonoid, prefix_sum::PrefixSum};

fn main() {
    let sums = PrefixSum::<DefaultMonoid<i64>>::from_iter([2, 3, 5]);
    assert_eq!(sums.query(1..3), 8);
}
