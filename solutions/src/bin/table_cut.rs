use std::iter;

use cp_library::{Cin, Cout, End};

fn solve(n: usize, m: usize, a: Vec<Vec<usize>>) -> (usize, String) {
    let tot = a
        .iter()
        .map(|row| row.iter().copied().sum::<usize>())
        .sum::<usize>();
    let opt = tot / 2;
    let mut curr = 0;
    let (last_i, last_j) = 'outer: {
        for i in 0..n {
            for j in (0..m).rev() {
                if curr == opt {
                    break 'outer (i, j);
                }
                curr += a[i][j];
            }
        }
        let mut res = String::new();
        res.extend(iter::repeat_n('D', n));
        res.extend(iter::repeat_n('R', m));
        return (opt * (tot - opt), res);
    };
    let mut res = String::new();
    res.extend(iter::repeat_n('D', last_i));
    res.extend(iter::repeat_n('R', last_j + 1));
    res.push('D');
    res.extend(iter::repeat_n('R', m - (last_j + 1)));
    res.extend(iter::repeat_n('D', n - (last_i + 1)));
    (opt * (tot - opt), res)
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n = cin.read();
        let m = cin.read();
        let mut a = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                a[i][j] = cin.read();
            }
        }
        let (opt, res) = solve(n, m, a);
        cout.println(opt);
        cout.println(res);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn smoke() {
//         assert_eq!(
//             solve(
//                 5,
//                 5,
//                 vec![
//                     vec![1, 0, 1, 1, 0],
//                     vec![0, 1, 0, 1, 1],
//                     vec![1, 0, 1, 0, 0],
//                     vec![0, 1, 0, 1, 0],
//                     vec![0, 0, 0, 0, 1],
//                 ],
//             ),
//             (0, String::from(""))
//         );
//     }
// }
