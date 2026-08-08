use cp_library::{Cin, Cout, End};

// finding all pairs of two
fn solve(h: Vec<usize>) -> usize {
    let n = h.len();
    let mut dp = vec![usize::MAX; n];
    dp[0] = h[0];
    for i in 1..dp.len() {
        dp[i] = dp[i].min(h[i].saturating_sub(i) + h[i - 1] + (if i > 1 { dp[i - 2] } else { 0 }));
        dp[i] = dp[i].min(h[i].saturating_sub(1) + dp[i - 1]);
    }
    dp[End]
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let h: Vec<usize> = cin.read_vec(n);
        let res = solve(h);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve(vec![3, 1, 4, 1, 2]), 7);
        assert_eq!(solve(vec![1, 1, 1, 1]), 1);
        assert_eq!(solve(vec![1, 2, 1, 3, 5, 2]), 7);
        assert_eq!(solve(vec![3, 1, 1, 3, 2, 1]), 5);
    }
}
