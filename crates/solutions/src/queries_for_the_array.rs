use std::panic;

use cp_library::{Cin, Cout, End, TestKind, driver};

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Sorted,
    Unknown,
    Unsorted,
}

// stack invariant: any number of sorted, followed by any number of unknown or unsorted
pub fn solve(s: &str) -> bool {
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

pub fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let s: String = cin.read();
    let res = solve(&s);
    cout.println(if res { "YES" } else { "NO" });
}

pub fn main() {
    driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use cp_library::test_driver;

    use super::*;

    #[test]
    pub fn smoke() {
        insta::assert_snapshot!(
            test_driver(
                run,
                TestKind::Many,
                "
            7
            ++1
            +++1--0
            +0
            0
            ++0-+1-+0
            ++0+-1+-0
            +1-+0
            "
            ),
            @"
        YES
        NO
        NO
        NO
        YES
        NO
        NO
        "
        );
    }
}
