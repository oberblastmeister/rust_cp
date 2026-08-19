use std::{thread, time::Duration};

use cp_library::prelude::*;

fn solve() {}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {}

pub fn main() {
    let mut v = vec![100, 108, 114, 118, 120, 5, 7, 19, 13, 11];
    let n = v.len() - 1;
    let mut count = 0;
    loop {
        dbg!(count);
        if count == 0 {
            for i in 1..(n - 1) {
                v[i] = v[i - 1] - v[i] + v[i + 1]
            }
            dbg!(&v);
            for i in (1..(n - 2)).rev() {
                v[i] = v[i - 1] - v[i] + v[i + 1]
            }
            dbg!(&v);
        } else {
            for i in 2..(n - 1) {
                v[i] = v[i - 1] - v[i] + v[i + 1]
            }
            dbg!(&v);
            for i in (1..(n - 2)).rev() {
                v[i] = v[i - 1] - v[i] + v[i + 1]
            }
            dbg!(&v);
        }
        thread::sleep(Duration::from_millis(200));
        count += 1;
    }
    // driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        main();
        //         assert_snapshot!(test_driver(run, TestKind::Many, "
        // "),
        //         @"
        // ")
    }
}
