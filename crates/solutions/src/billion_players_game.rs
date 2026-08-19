use cp_library::{algebra::DefaultMonoid, prefix_sum::PrefixSum, prelude::*};

fn solve(l: isize, r: isize, mut a: Vec<isize>) -> usize {
    a.sort_unstable();
    // let mut i = 0;
    // let mut j = a.len() - 1;
    // let mut res = 0;
    // dbg!(&a);
    // while i < j {
    //     dbg!(i, j, res);
    //     if a[i] < l && a[j] < l {
    //         res += l - a[i];
    //         res += l - a[j];
    //     } else if a[i] > r && a[j] > r {
    //         res += a[i] - r;
    //         res += a[j] - r;
    //     } else {
    //         res += a[j] - a[i];
    //         i += 1;
    //         j -= 1;
    //     }
    // }
    // res
    let mut mid = Vec::new();
    let mut ls: isize = 0;
    let mut rs: isize = 0;
    for &x in &a {
        if x < l || x > r {
            ls += (x.abs_diff(l)) as isize;
            rs += (x.abs_diff(r)) as isize;
        } else {
            mid.push(x as isize);
        }
    }
    mid.shrink_to_fit();
    let mid_sum: PrefixSum<DefaultMonoid<isize>> = PrefixSum::from_vec(mid.clone());
    dbg!(ls, rs, &mid, &mid_sum);
    if mid.len() % 2 == 1 {
        let mut res = 0;
        for i in 0..mid.len() {
            let curr = ((ls + (mid[i] - l)).min(rs - (r - mid[i])))
                .max((ls - (mid[i] - l)).min(rs + (r - mid[i])))
                + mid_sum.query(..i)
                + mid_sum.query((i + 1)..);
            res = res.max(curr);
            dbg!(res);
        }
        dbg!(res);
        res as usize
    } else {
        let curr =
            ls.min(rs) + (mid_sum.query((mid.len() / 2)..) - mid_sum.query(..(mid.len() / 2)));
        dbg!(curr);
        curr as usize
    }
    // dbg!(&a, &lt, &gt, &mid);
    // lt.reverse();
    // while !lt.is_empty() && !gt.is_empty() {
    //     res += gt.pop().unwrap() - lt.pop().unwrap();
    // }
    // dbg!(res);
    // let mut i = 0;
    // let mut j = mid.len() - 1;
    // while i < j {
    //     res += mid[j] - mid[i];
    //     i += 1;
    //     j -= 1;
    // }
    // dbg!(res);
    // if mid.len() % 2 == 1 {
    //     let k = mid.len() / 2;
    //     if let Some(x) = lt.pop() {
    //         res += mid[k] - x;
    //     } else if let Some(x) = gt.pop() {
    //         res += x - mid[k];
    //     }
    //     dbg!(res);
    // }
    // for &x in &lt {
    //     res += l - x;
    //     dbg!(res);
    // }
    // for &x in &gt {
    //     res += x - r;
    //     dbg!(res);
    // }
    // res
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let l = cin.read();
    let r = cin.read();
    let a = cin.read_vec(n);
    let res = solve(l, r, a);
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
        // assert_eq!(solve(6, 10, vec![9, 3, 1, 7, 5]), 13);
        //         assert_snapshot!(test_driver(run, TestKind::Many,
        // "
        // 4
        // 1 1 5
        // 3
        // 2 100 100
        // 50 200
        // 5 1 10
        // 5 7 3 9 1
        // 5 6 10
        // 9 3 1 7 5
        // "
        //         ), @"
        //         0
        //         150
        //         12
        //         12
        //         ");
    }
}
