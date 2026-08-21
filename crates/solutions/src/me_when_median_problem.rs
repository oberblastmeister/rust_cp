use cp_library::{binary_search, prelude::*};

fn check(a: &[usize], b: &[usize], k: usize) -> bool {
    let n = a.len();
    let mut res: isize = 0;
    let mut curr = 0;
    for i in 0..n {
        if a[i] < k && b[i] < k {
            curr += 1;
        } else if a[i] >= k && b[i] >= k {
            res -= curr.min(1) * 2;
            res += 2;
            curr = 0;
        }
    }
    res -= curr.min(1) * 2;
    res >= 2
}

fn solve(a: Vec<usize>, b: Vec<usize>) -> usize {
    let n = a.len();
    assert!(n == b.len());
    let bound = a.iter().copied().max().unwrap().max(b.iter().copied().max().unwrap());
    let res = binary_search::virtual_partition_point(0, bound + 1, |k| check(&a, &b, k));
    assert!(res > 0);
    res - 1
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n);
    let b = cin.read_vec(n);
    let res = solve(a, b);
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
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            6
            1
            1
            2
            3
            2 4 5
            1 3 6
            4
            7 5 4 8
            4 6 7 8
            8
            8 7 13 11 1 10 4 5
            11 11 12 8 9 2 3 13
            9
            16 1 9 12 5 18 10 10 16
            14 6 7 11 12 17 18 3 17
            6
            3 6 12 4 10 12
            2 3 2 7 8 9
"),
        @"
        1
        3
        6
        8
        14
        8
        ")
    }
}
