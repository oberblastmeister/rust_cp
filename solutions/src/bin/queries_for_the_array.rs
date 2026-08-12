use std::{collections::HashSet, iter, panic};

use cp_library::{Cin, Cout, End, Itertools};

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Sorted,
    Unknown,
    Unsorted,
}

// stack invariant: any number of sorted, followed by any number of unknown or unsorted
fn solve(s: &str) -> bool {
    use Item::*;
    let s = s.as_bytes();
    let mut stack = Vec::new();
    for &c in s {
        match c {
            b'+' => {
                stack.push(Unknown);
            }
            b'-' => {
                stack.pop().expect("invalid input");
            }
            b'1' => {
                for x in stack.iter_mut().rev() {
                    match x {
                        Unknown => *x = Sorted,
                        Sorted => break,
                        Unsorted => return false,
                    }
                }
            }
            b'0' => {
                if stack.len() <= 1 {
                    return false;
                }
                let x = &mut stack[End];
                match x {
                    Sorted => return false,
                    Unknown => *x = Unsorted,
                    Unsorted => {}
                };
            }
            _ => {
                panic!("invalid input");
            }
        }
    }
    true
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t: usize = cin.read();
    for _ in 0..t {
        let s: String = cin.read();
        let res = solve(&s);
        cout.println(if res { "YES" } else { "NO" });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve("++1"), true);
        assert_eq!(solve("+++1--0"), false);
        assert_eq!(solve("+0"), false);
        assert_eq!(solve("0"), false);
        assert_eq!(solve("++0-+1-+0"), true);
        assert_eq!(solve("++0+-1+-0"), false);
        assert_eq!(solve("+1-+0"), false);
    }
}
