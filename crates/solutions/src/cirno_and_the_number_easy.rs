use cp_library::prelude::*;

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

pub fn solve(a: usize, ds: &[usize]) -> usize {
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

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let a: usize = cin.read();
    let n: usize = cin.read();
    let ds: Vec<usize> = cin.read_vec(n);
    let res = solve(a, &ds);
    cout.println(res);
}

pub fn main() {
    driver(run, TestKind::Many);
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

pub fn brute(a: usize, ds: &[usize]) -> usize {
    let ads = to_digits(a);
    let mut gens = vec![];
    for n in 1..=(ads.len() + 1) {
        gens.extend(generate(n, &ds));
    }
    gens.iter().copied().map(|it| a.abs_diff(it)).min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn smoke() {}
}
