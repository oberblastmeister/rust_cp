use cp_library::{Cin, Cout, Itertools};
use std::iter;

// fn solve(i, j) {
// }
// fn solve(a: String, b: String) -> Option<usize> {
//     let a = a.into_bytes();
//     let b = b.into_bytes();
//     let mut pfa: Vec<usize> = iter::repeat(0).take(a.len() + 1).collect();
//     for i in 1..=a.len() {
//         pfa[i] = pfa[i - 1] + (a[i - 1] - b'0') as usize;
//     }
//     let mut pfb: Vec<usize> = iter::repeat(0).take(b.len() + 1).collect();
//     for i in 1..=b.len() {
//         pfb[i] = pfb[i - 1] + (b[i - 1] - b'0') as usize;
//     }
//     let mut i = 0;
//     let mut j = 0;
//     let mut k = 0;
//     while i < a.len() && j < b.len() {
//         let (i1, j1) = (i..=a.len())
//             .cartesian_product(j..=b.len())
//             .find(|(i1, j1)| {
//                 !(*i1 == i && *j1 == j) && (pfa[*i1] - pfa[i]) % 10 == (pfb[*j1] - pfb[j]) % 10
//             })?;
//         dbg!((i, i1, j, j1));
//         i = i1;
//         j = j1;
//         k += 1;
//     }
//     if ((pfa[a.len()] - pfa[i]) % 10) == ((pfb[b.len()] - pfb[j]) % 10) {
//         Some(k)
//     } else {
//         None
//     }
// }

// fn brute(a: String, b: String) -> Option<usize> {

// }

// fn main() {
//     let mut cin = Cin::new();
//     let mut cout = Cout::new();
//     let t: usize = cin.get();
//     for _ in 0..t {
//         let a: String = cin.get();
//         let b: String = cin.get();
//         match solve(a, b) {
//             None => cout.put(-1),
//             Some(it) => cout.put(it),
//         };
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn smoke() {
//         // assert_eq!(solve("5147".to_string(), "44441".to_string()), Some(2));
//         // assert_eq!(solve("2194".to_string(), "5602".to_string()), None);
//         // assert_eq!(solve("123450".to_string(), "012345".to_string()), Some(5));
//         assert_eq!(
//             solve("093252809".to_string(), "2004381".to_string()),
//             Some(3)
//         );
//         // assert_eq!(solve("09 3252809".to_string(), "20043 81".to_string()), Some(3));
//         // assert_eq!(solve("093 25 2809".to_string(), "2 0043 81".to_string()), Some(3));

//         // assert_eq!(solve("093 25 2809".to_string(), "2 0043 81".to_string()), Some(3));
//         //
//         // assert_eq!(solve("09 325 2809".to_string(), "20043 | 81".to_string()), Some(3));
//         // 3
//         // 043
//     }
// }

fn main() {
    
}