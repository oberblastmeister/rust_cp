use cp_library::{algebra::AddMonoid, prefix_sum::PrefixSum};

fn main() {
    let sums = PrefixSum::<AddMonoid<i64>>::from_iter([2, 3, 5]);
    assert_eq!(sums.query(1..3), 8);
}
