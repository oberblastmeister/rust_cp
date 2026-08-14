use cp_library::prelude::*;

pub fn solve(n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut next: Vec<usize> = vec![n; n + 1];
    let mut next_start: Vec<usize> = vec![n; n + 1];
    let mut next_for: Vec<usize> = vec![n; n + 2];
    let mut res = 0;
    for i in (0..n).rev() {
        next_start[i] = next_start[i + 1];
        if a[i] == 1 || b[i] == 1 {
            next_start[i] = i;
        }
        if a[i] == b[i] {
            next[i] = next[next_for[a[i] + 1]]
        } else {
            next[i] = i;
        }
        next_for[a[i]] = i;
        next_for[b[i]] = i;
        res += next[next_start[i]] - i;
    }
    res
}

pub fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n);
    let b = cin.read_vec(n);
    let res = solve(n, a, b);
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
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            4
            3
            1 2 1
            1 2 2
            2
            2 1
            1 2
            5
            1 3 2 1 4
            1 4 2 3 2
            9
            1 1 3 1 1 3 2 3 1
            1 3 1 1 3 1 2 1 3
"), @"
        4
        0
        7
        12
        ")
    }
}
