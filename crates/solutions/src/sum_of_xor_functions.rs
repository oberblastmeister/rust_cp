use cp_library::{mod_arith::ModUsize, prelude::*};

const MOD: usize = 998244353;
type musize = ModUsize<MOD>;

fn solve_at(a: &[usize], d: u32) -> musize {
    let n = a.len();
    let mut dp_count = [0usize; 2];
    let mut dp_sum = [musize::new(0); 2];
    let mut res = musize::new(0);
    for i in 1..=n {
        let b = (a[i - 1] >> d) & 1;
        if b == 0 {
            dp_sum = [dp_sum[0] + dp_count[0] + 1, dp_sum[1] + dp_count[1]];
        } else {
            dp_sum = [dp_sum[1] + dp_count[1], dp_sum[0] + dp_count[0] + 1];
        }
        if b == 0 {
            dp_count = [dp_count[0] + 1, dp_count[1]];
        } else {
            dp_count = [dp_count[1], dp_count[0] + 1];
        }
        res += dp_sum[1];
    }
    res
}

fn solve(a: Vec<usize>) -> musize {
    let width = a.iter().max().unwrap().checked_ilog2().unwrap_or(0) + 1;
    let mut res = musize::new(0);
    for d in 0..width {
        res += solve_at(&a, d) * (1 << d);
    }
    res
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n);
    let res = solve(a);
    cout.println(res);
}

pub fn main() {
    driver(run, TestKind::Single);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Single, "
            3
            1 3 2
"),
        @"12");
        assert_snapshot!(test_driver(run, TestKind::Single, "
            4
            39 68 31 80
"), @"1337");
        assert_snapshot!(test_driver(run, TestKind::Single, "
            7
            313539461 779847196 221612534 488613315 633203958 394620685 761188160
"), @"257421502");
    }
}
