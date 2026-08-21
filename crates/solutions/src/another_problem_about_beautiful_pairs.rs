use cp_library::prelude::*;

pub fn solve(a: Vec<usize>) -> usize {
    let n = a.len();
    let sqn = n.isqrt();
    let mut res = 0;
    for i in 0..n {
        for aj in 1..=sqn {
            let lhs = a[i] * aj;
            let j = lhs + i;
            if j >= n {
                break;
            }
            if a[j] != aj {
                continue;
            }
            res += 1;
        }
        if a[i] > sqn {
            let j = i;
            let aj = a[i];
            for ai in 1..=sqn {
                let lhs = ai * aj;
                if lhs > j {
                    break;
                }
                let i = j - lhs;
                if a[i] != ai {
                    continue;
                }
                res += 1;
            }
        }
    }
    res
}

pub fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n: usize = cin.read();
    let a: Vec<usize> = cin.read_vec(n);
    let res = solve(a);
    cout.println(res);
}

pub fn main() {
    driver(run, TestKind::Many)
}

#[cfg(test)]
mod tests {
    use cp_library::test_driver;
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            4
            5
            1 1 2 100 4
            6
            2 2 1 1 2 2
            10
            1 1 2 3 4 1 1 7 3 9
            2
            1000000000 1000000000
"),
        @"
        3
        7
        10
        0
        ")
    }
}
