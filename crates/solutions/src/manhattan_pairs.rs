use cp_library::prelude::*;

fn solve(a: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut a = a.into_iter().enumerate().collect_vec();
    let n = a.len();
    a.sort_unstable_by_key(|&(_, (x, _))| x);
    a[..(n / 2)].sort_unstable_by_key(|&(_, (_, y))| y);
    a[(n / 2)..].sort_unstable_by_key(|&(_, (_, y))| y);
    let mut i = 0;
    let mut j = n;
    let mut res = vec![];
    while i < j {
        res.push((a[i].0, a[j - 1].0));
        i += 1;
        j -= 1;
    }
    res
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a: Vec<(isize, isize)> = (0..n).map(|_| (cin.read(), cin.read())).collect();
    let res = solve(a);
    for &(x, y) in &res {
        cout.print(x + 1);
        cout.print(' ');
        cout.print(y + 1);
        cout.print('\n');
    }
}

pub fn main() {
    driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            2
            4
            1 1
            3 0
            4 2
            3 4
            10
            -1 -1
            -1 2
            -2 -2
            -2 0
            0 2
            2 -3
            -4 -4
            -4 -2
            0 1
            -4 -2
"),
        @"
        2 4
        1 3
        7 5
        8 2
        10 9
        3 1
        4 6
        ")
    }
}
