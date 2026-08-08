use std::{collections::HashSet, iter};

use cp_library::{Cin, Cout, End, Itertools};

fn to_digits(x: usize) -> Vec<usize> {
    x.to_string()
        .as_bytes()
        .into_iter()
        .map(|b| (b - b'0') as usize)
        .collect()
}

fn from_digits(ds: &[usize]) -> usize {
    assert!(!ds.is_empty());
    assert!(ds.iter().copied().all(|d| d <= 9));
    ds.into_iter().copied().fold(0, |acc, d| acc * 10 + d)
}

fn solve(a: usize, ds: &[usize]) -> usize {
    assert!(ds.is_sorted());
    assert!(ds.iter().copied().all(|d| d <= 9));
    let ds_set: HashSet<usize> = ds.iter().copied().collect();
    let ads = to_digits(a);
    let mut res = usize::MAX;
    let mut add = |digits: &[usize]| {
        res = res.min(a.abs_diff(from_digits(digits)));
    };
    if ads.len() > 1 {
        let sim = vec![ds[End]; ads.len() - 1];
        add(&sim)
    }
    {
        let mut sim = vec![if ds[0] == 0 && ds.len() > 1 {
            ds[1]
        } else {
            ds[0]
        }];
        sim.extend(iter::repeat_n(ds[0], ads.len()));
        add(&sim);
    }
    'outer: {
        let mut sim: Vec<usize> = Vec::new();
        for (i, ad) in ads.iter().copied().enumerate() {
            for &d in ds {
                if d < ad {
                    let mut sim = sim.clone();
                    sim.push(d);
                    sim.extend(iter::repeat_n(ds[End], ads.len() - (i + 1)));
                    add(&sim);
                } else if ad < d {
                    let mut sim = sim.clone();
                    sim.push(d);
                    sim.extend(iter::repeat_n(ds[0], ads.len() - (i + 1)));
                    add(&sim);
                }
            }
            if ds_set.contains(&ad) {
                sim.push(ad);
            } else {
                break 'outer;
            }
        }
        add(&sim);
    }
    res
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let a: usize = cin.read();
        let n: usize = cin.read();
        let ds: Vec<usize> = cin.read_vec(n);
        let res = solve(a, &ds);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {}
}
