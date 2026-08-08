#![no_main]

use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;

use cp_library::{Cin, Cout, End, Itertools};
use std::{collections::HashSet, iter};

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
    ds.into_iter()
        .copied()
        .skip(1)
        .fold(ds[0], |acc, d| acc * 10 + d)
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
        dbg!(&sim);
        add(&sim)
    }
    {
        let mut sim = vec![if ds[0] == 0 && ds.len() > 1 {
            ds[1]
        } else {
            ds[0]
        }];
        sim.extend(iter::repeat_n(ds[0], ads.len()));
        dbg!(&sim);
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
                    dbg!(&sim);
                    add(&sim);
                } else if d == ad {
                    sim.push(ad);
                } else {
                    assert!(ad < d);
                    let mut sim = sim.clone();
                    sim.push(d);
                    sim.extend(iter::repeat_n(ds[0], ads.len() - (i + 1)));
                    dbg!(&sim);
                    add(&sim);
                }
            }
            if !ds_set.contains(&ad) {
                break 'outer;
            }
        }
        dbg!(&sim);
        add(&sim);
    }
    res
}

fn generate(n: usize, ds: &[usize]) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut res: Vec<usize> = Vec::new();
    for d in ds {
        res.extend(generate(n - 1, ds).into_iter().map(|it| it * 10 + d))
    }
    res
}

fn brute(a: usize, ds: &[usize]) -> usize {
    let ads = to_digits(a);
    let mut gens = vec![];
    for n in 1..=(ads.len() + 1) {
        gens.extend(generate(n, &ds));
    }
    gens.iter().copied().map(|it| a.abs_diff(it)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve(0, &vec![0]), 0);
        assert_eq!(solve(11, &vec![1, 2]), 0);
        assert_eq!(solve(222, &vec![3, 4]), 111);
        assert_eq!(solve(3333, &vec![6, 7]), 2556);
    }
}

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);
    let Ok((a, mut ds)) = unstructured.arbitrary::<(usize, Vec<usize>)>() else {
        return;
    };
    if a > 1000 as usize {
        return;
    }
    if ds.is_empty() {
        return;
    }
    if !ds.iter().copied().all(|d| d <= 9) {
        return;
    }
    ds.sort();
    let ds: Vec<usize> = ds.into_iter().unique().collect();
    dbg!(a, &ds);
    let res1 = solve(a, &ds);
    let res2 = brute(a, &ds);
    assert_eq!(res1, res2);
    // fuzzed code goes here
});
