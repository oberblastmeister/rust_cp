use cp_library::{
    Cin, Cout, End, Frac, Itertools,
    frac::F,
    mod_arith::{self, FactTable},
};

const MOD: usize = 998244353;

type musize = cp_library::ModUsize<MOD>;

pub fn M(x: usize) -> musize {
    musize::new(x)
}

pub fn solve(n: usize, a: Vec<usize>, b: Vec<usize>) -> musize {
    let mut pairs = (0..n)
        .cartesian_product(0..n)
        .filter(|(i, j)| i != j)
        .map(|(i, j)| F(b[i]) / F(b[j]))
        .collect_vec();
    pairs.sort_unstable();
    let fact = FactTable::new(n);
    let mut res = M(0);
    for i in 0..n {
        for j in (i + 1)..n {
            let k = F(a[j]) / F(a[i]);
            let pi = pairs.partition_point(|&x| x <= k);
            let num = pairs[pi..].len();
            let rest = fact[n - 2];
            res += num * rest;
        }
    }
    res / fact[n]
}

pub fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let a: Vec<usize> = cin.read_vec(n);
        let b: Vec<usize> = cin.read_vec(n);
        let res = solve(n, a, b);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn smoke() {
        assert_eq!(solve(5, vec![1, 14, 5, 1, 4], vec![1, 1, 1, 1, 1]), M(5));
        assert_eq!(solve(3, vec![3, 2, 5], vec![3, 2, 5]), M(665496236));
        assert_eq!(
            solve(
                10,
                vec![10, 72, 65, 43, 73, 23, 78, 13, 49, 99],
                vec![31, 90, 45, 19, 44, 18, 59, 31, 48, 29]
            ),
            M(820778710)
        );
    }
}
