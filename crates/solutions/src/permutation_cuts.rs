use cp_library::{mod_arith::ModUsize, prelude::*};

const MOD: usize = 998244353;
type musize = ModUsize<MOD>;
fn M(x: usize) -> musize {
    musize::new(x)
}

fn solve(n: usize, a: Vec<usize>) -> musize {
    if a.contains(&n) {
        return M(0);
    }
    let Some(i) = a.iter().position(|&x| x == n - 1) else {
        return M(0);
    };
    let j = i + a[i..].iter().copied().take_while(|&x| x == n - 1).count();
    if a[j..].contains(&(n - 1)) {
        return M(0);
    }
    if !a[..i].is_sorted() {
        return M(0);
    }
    if !a[j..].is_sorted_by(|x, y| x >= y) {
        return M(0);
    }
    let left_set: HashSet<usize> = a[..i].iter().copied().collect();
    let right_set: HashSet<usize> = a[j..].iter().copied().collect();
    if !left_set.is_disjoint(&right_set) {
        return M(0);
    }
    let mut nums = a.clone();
    nums.sort_unstable();
    let mut res = M(1);
    let mut curr = 0;
    let mut have = 0;
    for x in nums {
        assert!(x >= curr);
        if x > curr {
            have += x - curr - 1;
            curr = x;
        } else {
            if have > 0 {
                res *= M(have);
                have -= 1;
            } else {
                return M(0);
            }
        }
    }
    assert_eq!(have, 0);
    return res * 2;
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n - 1);
    let res = solve(n, a);
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
        assert_eq!(solve(5, vec![4, 1, 2, 4]), M(0));
    }

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
                    11
                    2
                    1
                    3
                    2 2
                    3
                    1 1
                    4
                    2 3 2
                    5
                    3 3 4 2
                    2
                    2
                    3
                    1 2
                    4
                    3 3 3
                    5
                    4 4 4 4
                    4
                    2 1 2
                    6
                    3 3 5 5 5
        "),
                @"
        2
        2
        0
        0
        2
        0
        2
        4
        12
        0
        8
        ")
    }
}

