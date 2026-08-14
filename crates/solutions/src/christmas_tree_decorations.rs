use std::iter;

use cp_library::{Cin, Cout, End, mod_arith::FactTable, mod_arith::ModUsize};

const MOD: usize = 998244353;
type musize = ModUsize<MOD>;
pub fn M(x: usize) -> musize {
    musize::new(x)
}

pub fn solve(n: usize, a: Vec<usize>) -> musize {
    let &largest = a[1..].iter().max().unwrap();
    let necessary = a[1..].iter().copied().map(|x| (largest - x).saturating_sub(1)).sum::<usize>();
    if necessary > a[0] {
        return M(0);
    }
    let fact = FactTable::new(n);
    let left = a[0] - necessary;
    let num_at_max = a[1..].iter().filter(|&&x| x == largest).count();
    let mut res = M(0);
    for k in 0..=left.min(n - num_at_max) {
        let curr = num_at_max
            * fact.choose(n - num_at_max, k)
            * fact[num_at_max - 1 + k]
            * fact[n - num_at_max - k];
        res += curr;
    }
    res
}

pub fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t: usize = cin.read();
    for _ in 0..t {
        let n = cin.read();
        let a = cin.read_vec(n + 1);
        let res = solve(n, a);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn smoke() {
        assert_eq!(solve(3, vec![1, 2, 1, 0]), M(2));
        assert_eq!(solve(3, vec![1, 0, 2, 0]), M(0));
        assert_eq!(solve(1, vec![2, 5]), M(1));
        assert_eq!(solve(4, vec![6, 1, 4, 2, 1]), M(12));
    }
}
