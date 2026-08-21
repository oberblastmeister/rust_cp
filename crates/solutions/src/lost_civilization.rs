use cp_library::prelude::*;

fn solve(a: Vec<usize>) -> usize {
    let n = a.len();
    let mut stack = vec![a[0]];
    let mut res = 1;
    for i in 1..n {
        while let Some(&x) = stack.last()
            && x + 1 != a[i]
        {
            stack.pop();
        }
        if stack.is_empty() {
            res += 1;
        }
        stack.push(a[i]);
    }
    res
}

fn solve2(a: Vec<usize>) -> usize {
    let n = a.len();
    let mut stack = vec![(0, usize::MAX)];
    let mut res = 0;
    let mut curr = 0;
    for i in 0..n {
        let mut tot_popped = 0;
        while let (popped, x) = stack[End]
            && x != usize::MAX
            && x + 1 != a[i]
        {
            tot_popped += popped + 1;
            stack.pop();
        }
        stack[End].0 += tot_popped;
        curr += stack[End].0 + 1;
        res += curr;
        stack.push((0, a[i]));
    }
    res
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let a = cin.read_vec(n);
    let res = solve2(a);
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
    fn case() {
        assert_eq!(solve2(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn smoke() {
        assert_snapshot!(test_driver(run, TestKind::Many, "
            5
            5
            1 2 3 4 5
            5
            1 3 5 7 9
            5
            1 2 5 6 5
            7
            1 2 4 5 3 7 8
            9
            9 8 9 2 3 4 4 5 3
"),
        @"
        15
        35
        25
        60
        78
        ")
    }
}
