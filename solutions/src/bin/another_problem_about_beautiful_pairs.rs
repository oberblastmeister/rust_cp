use std::{collections::HashSet};

use cp_library::{Cin, Cout, End};

fn solve(a: Vec<usize>) -> usize {
    dbg!(&a);
    let n = a.len();
    let mut res = 0;
    for j in 1..n {
        dbg!(j);
        let mut factors = HashSet::new();
        for d in 1..=(j.isqrt()) {
            factors.insert((d, j / d));
            factors.insert((j / d, d));
        }
        dbg!(&factors);
        for (d1, d2) in factors {
            if d2 != a[j] {
                continue;
            }
            let ai = d1;
            dbg!(ai);
            if ai * a[j] > j {
                continue;
            }
            let i = j - ai * a[j];
            if a[i] != ai {
                dbg!("wrong", (i, j));
                continue;
            }
            dbg!("yes", (i, j));
            assert!((j - i) == a[j] * a[i]);
            res += 1;
        }
    }
    res
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let a: Vec<usize> = cin.read_vec(n);
        let res = solve(a);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve(vec![1, 1, 2, 100, 4]), 3);
        assert_eq!(solve(vec![2, 2, 1, 1, 2, 2]), 7);
    }
}
