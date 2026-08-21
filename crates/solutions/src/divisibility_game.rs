use cp_library::prelude::*;

fn solve(mut a: Vec<usize>, b: Vec<usize>) -> bool {
    let n = a.len();
    a.sort_unstable();
    a.dedup();
    let m = b.len();
    let mut count = vec![0; n + m + 1];
    for &x in &a {
        let mut y = x;
        while y <= n + m {
            count[y] += 1;
            y += x;
        }
    }
    for &c in &count {
        assert!(c <= a.len());
    }
    let mut num_a = 0;
    let mut num_b = 0;
    let mut num_ab = 0;
    for &x in &b {
        if count[x] == a.len() {
            num_a += 1;
        } else if count[x] == 0 {
            num_b += 1;
        } else {
            num_ab += 1;
        }
    }
    if num_ab % 2 == 0 { num_a > num_b } else { num_a >= num_b }
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let m = cin.read();
    let a = cin.read_vec(n);
    let b = cin.read_vec(m);
    let res = solve(a, b);
    cout.println(if res { "Alice" } else { "Bob" });
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
            3
            9 3
            3 2 4 2 2 4 4 2 4
            6 7 12
            10 3
            3 2 5 4 2 5 3 4 4 4
            10 7 13
            1 5
            1
            1 2 3 4 5
"),
        @"
        Alice
        Bob
        Alice
        ")
    }
}
